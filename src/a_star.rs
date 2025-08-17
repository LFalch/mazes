use super::*;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::collections::hash_map::Entry::*;

#[inline]
const fn heuristic(p: Point, goal: Point) -> Unit {
    let Point(x, y) = p;
    let Point(i, j) = goal;

    i.abs_diff(x) + j.abs_diff(y)
}

struct AStarNode {
    f_cost: Unit,
    point: Point,
    g_cost: Unit,
}

impl PartialEq for AStarNode {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f_cost == other.f_cost
    }
}
impl Eq for AStarNode {}
impl PartialOrd for AStarNode {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for AStarNode {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost.cmp(&self.f_cost)
    }
}

pub fn solve(maze: &Maze) -> Option<(Unit, Vec<Point>)> {
    let start = maze.get_entrance();
    let end = maze.get_exit();

    let mut closed_set = HashMap::<Point, (Point, Unit)>::new();
    let mut open_set = BinaryHeap::new();
    open_set.push(AStarNode {
        f_cost: heuristic(start, end),
        point: start,
        g_cost: 0,
    });

    while let Some(AStarNode { g_cost, point, .. }) = open_set.pop() {
        if point == end {
            let mut path = vec![point];
            while let Some((parent, _)) = closed_set.remove(path.last().unwrap()) {
                path.push(parent);
            }
            path.reverse();

            return Some((g_cost, path));
        }

        // If this node already exists with a lower G cost, skip it
        if let Some(&(_, g)) = closed_set.get(&point) && g_cost > g {
            continue;
        }
        for neighbour in point.neighbours(maze) {
            let g_cost = g_cost + 1;

            match closed_set.entry(neighbour) {
                Vacant(e) => {
                    e.insert((point, g_cost));
                }
                Occupied(mut e) => {
                    if e.get().1 > g_cost {
                        e.insert((point, g_cost));
                    } else {
                        continue;
                    }
                }
            }

            open_set.push(AStarNode {
                f_cost: g_cost + heuristic(neighbour, end),
                g_cost,
                point: neighbour,
            })
        }
    }

    // no solution
    None
}
