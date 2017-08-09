use std::iter::FromIterator;
use std::ops::{Index, IndexMut};

mod a_star;

pub use a_star::*;

#[derive(Debug, Clone)]
pub struct Maze {
    width: usize,
    points: Vec<bool>
}

impl Maze {
    pub fn get(&self, x: usize, y: usize) -> Option<&bool> {
        if x >= self.width {
            return None;
        }
        self.points.get(x + y * self.width)
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut bool> {
        if x >= self.width {
            return None;
        }
        self.points.get_mut(x + y * self.width)
    }
    pub fn get_entrance(&self) -> usize {
        self.points.iter().take(self.width).position(|&p| !p).unwrap()
    }
    pub fn get_exit(&self) -> usize {
        self.width - 1 - self.points.iter().rev().take(self.width).position(|&p| !p).unwrap()
    }
    #[deprecated]
    pub fn is_node(&self, x: usize, y: usize) -> bool {
        if self[(x, y)] {
            return false
        }
        let (_, height) = self.dimensions();
        if y == 0 || y == height-1 {
            return true;
        }
        let mut ret = false;

        if !self[(x-1, y)] {
            ret = true;
        }
        if !self[(x+1, y)] {
            ret = !ret;
        }
        if ret {return true}
        if !self[(x, y-1)] {
            ret = true;
        }
        if !self[(x, y+1)] {
            ret = !ret;
        }
        ret
    }
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.points.len() / self.width)
    }
}

impl Index<(usize, usize)> for Maze {
    type Output = bool;
    fn index(&self, (x, y): (usize, usize)) -> &bool {
        self.get(x, y).unwrap()
    }
}

impl IndexMut<(usize, usize)> for Maze {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut bool {
        self.get_mut(x, y).unwrap()
    }
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Debug, Clone)]
pub struct Point (pub usize, pub usize);

impl Point {
    pub fn neighbours(&self, maze: &Maze) -> Vec<Point>{
        let mut poses = vec![(self.0+1, self.1), (self.0, self.1+1)];
        if self.0 > 0 {
            poses.push((self.0-1, self.1));
        }
        if self.1 > 0 {
            poses.push((self.0, self.1-1));
        }
        poses
            .into_iter()
            .map(|(x, y)| Point(x, y))
            .filter(|&Point(x, y)| !(maze.get(x, y).cloned().unwrap_or(true)))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct MazeBuilder {
    points: Vec<bool>
}

impl FromIterator<bool> for MazeBuilder {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item=bool> {
        MazeBuilder{
            points: iter.into_iter().collect()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MazeError {
    NotOneEntrance,
    NotOneExit,
    NotBounded
}

impl MazeBuilder {
    pub fn finish(self, width: usize) -> Result<Maze, MazeError> {
        let MazeBuilder{points} = self;
        assert!(points.len() % width == 0);

        if points.iter().take(width).filter(|p| !**p).count() != 1 {
            Err(MazeError::NotOneEntrance)
        } else if points.iter().rev().take(width).filter(|p| !**p).count() != 1 {
            Err(MazeError::NotOneExit)
        } else if points.iter().enumerate()
            .filter(|&(i, &p)| (i % width == 0 || (i+1) % width == 0) && !p).count() != 0 {
            Err(MazeError::NotBounded)
        } else {
            Ok(Maze{points, width})
        }
    }
}
