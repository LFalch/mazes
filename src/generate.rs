use std::collections::HashSet;

use image::{GrayImage, Luma};
use rand::{random_bool, random_range, rng, seq::SliceRandom};

use crate::{Point, Unit};

const PASS: Luma<u8> = Luma([255]);
const WALL: Luma<u8> = Luma([0]);
const DIRS: [Direction; 4] = [Down, Up, Right, Left];

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}
impl Direction {
    pub const fn neighbour(self, Point(x, y): Point, (w, h): (Unit, Unit)) -> Option<Point> {
        match self {
            Right if x >= w - 1 => None,
            Right => Some(Point(x + 1, y)),
            Down if y >= h - 1 => None,
            Down => Some(Point(x, y + 1)),
            Up if y == 0 => None,
            Up => Some(Point(x, y - 1)),
            Left if x == 0 => None,
            Left => Some(Point(x - 1, y)),
        }
    }
    pub fn cell(self, Point(x, y): Point, (w, h): (Unit, Unit)) -> Option<(u32, u32)> {
        match self {
            Right if x >= w => None,
            Right => (1 + x * 2).checked_add(2).map(|x| (x, 1 + y * 2)),
            Down if y >= h => None,
            Down => (1 + y * 2).checked_add(2).map(|y| (1 + x * 2, y)),
            Up => (1 + y * 2).checked_sub(2).map(|y| (1 + x * 2, y)),
            Left => (1 + x * 2).checked_sub(2).map(|x| (x, 1 + y * 2)),
        }
    }
    pub fn wall(self, Point(x, y): Point, (w, h): (Unit, Unit)) -> Option<(u32, u32)> {
        match self {
            Right if x >= w => None,
            Right => (1 + x * 2).checked_add(1).map(|x| (x, 1 + y * 2)),
            Down if y >= h => None,
            Down => (1 + y * 2).checked_add(1).map(|y| (1 + x * 2, y)),
            Up => (1 + y * 2).checked_sub(1).map(|y| (1 + x * 2, y)),
            Left => (1 + x * 2).checked_sub(1).map(|x| (x, 1 + y * 2)),
        }
    }
    pub fn random_dirs() -> [Self; 4] {
        let mut dirs = DIRS;
        if random_bool(0.5) {
            dirs.swap(0, 1);
        }
        if random_bool(0.3) {
            dirs.swap(0, random_range(1..4));
        }
        if random_bool(0.3) {
            dirs.swap(1, random_range(2..4));
        }
        if random_bool(0.5) {
            dirs.swap(2, 3);
        }
        dirs
    }
}

use self::Direction::*;

pub fn backtrack(width: Unit, height: Unit) -> GrayImage {
    let mut img = GrayImage::new(width*2+1, height*2+1);

    let entrance_x = random_range(0..width);
    img.put_pixel(entrance_x * 2 + 1, 0, PASS);
    img.put_pixel(entrance_x * 2 + 1, 1, PASS);

    let entrance_point = Point(entrance_x, 0);
    let mut stack = vec![entrance_point];
    let mut visited: HashSet<Point> = HashSet::with_capacity(width as usize * height as usize);
    visited.insert(entrance_point);

    while let Some(p) = stack.pop() {
        let dirs = Direction::random_dirs();

        for dir in dirs {
            let Some(neighbour) = dir.neighbour(p, (width, height)) else {
                continue;
            };
            if visited.contains(&neighbour) {
                continue;
            }
            stack.push(p);
            visited.insert(neighbour);
            stack.push(neighbour);

            let (px, py) = dir.wall(p, (width, height)).unwrap();
            img.put_pixel(px, py, PASS);
            let (px, py) = dir.cell(p, (width, height)).unwrap();
            img.put_pixel(px, py, PASS);

        }
    }

    let exit_x = random_range(0..width);
    img.put_pixel(exit_x * 2 + 1, height * 2, PASS);

    img
}

pub fn primms(width: Unit, height: Unit) -> GrayImage {
    let (w, h) = (width*2+1, height*2+1);
    let mut img = GrayImage::new(w, h);

    let start_point = Point(1 + random_range(0..width) * 2, 1 + random_range(0..height) * 2);
    img.put_pixel(start_point.0, start_point.1, PASS);

    let mut wall_list: Vec<_> = start_point
        .neighbour_points()
        .filter(|p| !p.is_edge(w, h))
        .collect();

    'wall_loop: while !wall_list.is_empty() {
        let wall = wall_list.swap_remove(random_range(0..wall_list.len()));

        let mut visited_cell = None;
        let iter = wall.neighbour_points()
            .filter(|p| !p.is_edge(w, h));
        for n_cell in iter {
            if img.get_pixel(n_cell.0, n_cell.1) == &PASS {
                if visited_cell.is_some() {
                    continue 'wall_loop;
                }
                visited_cell = Some(n_cell);
            }
        }
        let Some(visited_cell) = visited_cell else {
            continue 'wall_loop;
        };
        img.put_pixel(wall.0, wall.1, PASS);
        let unvisited_cell = wall + (wall - visited_cell);
        img.put_pixel(unvisited_cell.0, unvisited_cell.1, PASS);
        wall_list.extend(unvisited_cell.neighbour_points()
            .filter(|p| img.get_pixel(p.0, p.1) == &WALL)
            .filter(|p| !p.is_edge(w, h))
        );
    }

    let entrance_x = 1 + random_range(0..width) * 2;
    img.put_pixel(entrance_x, 0, PASS);
    let exit_x = 1 + random_range(0..width) * 2;
    img.put_pixel(exit_x, h - 1, PASS);

    img
}