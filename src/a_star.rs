use super::*;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::collections::hash_map::Entry::*;

fn heuristic(p: &Point, goal: &Point) -> usize {
    let &Point(x, y) = p;
    let &Point(i, j) = goal;

    pos_diff(i, x) + pos_diff(j, y)
}

#[inline]
fn pos_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

struct AStarNode {
    f_cost: usize,
    point: Point,
    g_cost: usize
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool{
        self.f_cost == other.f_cost
    }
}

impl Eq for AStarNode {}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f_cost.partial_cmp(&self.f_cost)
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost.cmp(&self.f_cost)
    }
}

pub fn solve(maze: &Maze) -> Option<(usize, Vec<Point>)> {
    let start = maze.get_entrance().into();
    let end = maze.get_exit().into();

    let mut closed_set = HashMap::<Point, (Point, usize)>::new();
    let mut open_set = BinaryHeap::new();
    open_set.push(AStarNode{
        f_cost: heuristic(&start, &end),
        point: start,
        g_cost: 0,
    });

    while let Some(AStarNode{ g_cost, point, ..}) = open_set.pop() {
        if point == end {
            let mut parents: HashMap<_, _> = closed_set.into_iter().map(|(n, (p, _))| (n, p)).collect();
            let mut path = vec![point];
            while let Some(parent) = parents.remove(path.last().unwrap()) {
                path.push(parent);
            }
            path.reverse();

            return Some((g_cost, path))
        }

        // If this node also exists with a lower G cost, skip this one
        if let Some(&(_, g)) = closed_set.get(&point) {
            if g_cost > g {
                continue
            }
        }
        for neighbour in point.neighbours(&maze) {
            let g_cost = g_cost + 1;
            let push_to_open_set;

            match closed_set.entry(neighbour.clone()) {
                Vacant(e) => {
                    e.insert((point.clone(), g_cost));
                    push_to_open_set = true;
                }
                Occupied(mut e) => if e.get().1 > g_cost {
                    e.insert((point.clone(), g_cost));
                    push_to_open_set = true;
                } else {
                    push_to_open_set = false;
                }
            }

            if push_to_open_set {
                open_set.push(AStarNode{
                    f_cost: g_cost + heuristic(&neighbour, &end),
                    g_cost,
                    point: neighbour
                })
            }
        }
    }

    // no solution
    None
}
