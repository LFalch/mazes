use super::*;

use self::under::*;

mod under {
    use super::*;
    use std::cmp::{Ordering, Reverse};
    use std::cell::Cell;

    pub struct Cmp<T: PartialOrd + PartialEq>(T);

    impl<T: PartialEq + PartialOrd> PartialEq for Cmp<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0.eq(&other.0)
        }
    }

    impl<T: PartialOrd + PartialEq> PartialOrd for Cmp<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl<T: PartialOrd + PartialEq> Eq for Cmp<T> {}

    impl<T: PartialOrd + PartialEq> Ord for Cmp<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).expect("Poisoned")
        }
    }

    #[derive(Clone)]
    pub struct AStarPoint {
        pub inner: Point,
        pub came_from: Option<Point>,
        g_score: f64,
        h_score: f64,
        f_score: Cell<f64>
    }

    #[inline]
    fn pos_diff(a: usize, b: usize) -> usize {
        if a > b {
            a - b
        } else {
            b - a
        }
    }

    fn heuristic(p: &Point, goal: &Point) -> f64 {
        let &Point(x, y) = p;
        let &Point(i, j) = goal;

        (pos_diff(i, x) as f64).hypot(pos_diff(j, y) as f64)
    }

    impl AStarPoint {
        pub fn from_point(p: Point, g: f64, end: &Point) -> Self {
            AStarPoint {
                h_score: heuristic(&p, end),
                came_from: None,
                g_score: g as f64,
                f_score: Cell::new(0./0.),
                inner: p,
            }
        }
        pub fn g(&self) -> &f64 {
            &self.g_score
        }
        pub fn f_score(&self) -> Reverse<Cmp<f64>> {
            if self.f_score.get().is_nan() {
                self.f_score.set((self.g_score + self.h_score) * 10.)
            }
            Reverse(Cmp(self.f_score.get()))
        }
    }

    impl PartialEq<Self> for AStarPoint {
        fn eq(&self, other: &AStarPoint) -> bool {
            &self.inner == &other.inner
        }
    }

    impl Eq for AStarPoint {}

    impl PartialEq<Point> for AStarPoint {
        fn eq(&self, other: &Point) -> bool {
            &self.inner == other
        }
    }
}

pub fn a_star(start: Point, end: Point, grid: Maze) -> Option<Vec<Point>> {
    let start = AStarPoint::from_point(start, 0., &end);

    let mut closed_set = Vec::new();
    let mut open_set = vec![start];

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap();

        if current == end {
            let AStarPoint{inner, mut came_from, ..} = current;
            let mut path = vec![inner];

            loop {
                let i = if let Some(ref p) = came_from {
                    closed_set.iter().position(|a| a == p).unwrap()
                } else {
                    return Some(path)
                };

                let current = closed_set.remove(i);

                let AStarPoint{inner, came_from: cf, ..} = current;
                path.push(inner);
                came_from = cf;
            }
        }

        for neighbour in current.inner.neighbours(&grid) {
            let mut neighbour = AStarPoint::from_point(neighbour, *current.g() + 1., &end);
            if closed_set.contains(&neighbour) {
                continue;
            }

            neighbour.came_from = Some(current.inner.clone());

            if open_set.contains(&neighbour) {
                let orig = open_set.iter_mut().find(|p| &**p == &neighbour).unwrap();
                if *neighbour.g() < *orig.g() {
                    *orig = neighbour;
                }
            } else {
                let index = open_set.binary_search_by_key(&neighbour.f_score(),
                    AStarPoint::f_score).unwrap_or_else(|x| x);

                open_set.insert(index, neighbour);
                // open_set.push(neighbour)
            }
        }

        closed_set.push(current);
        // open_set.sort_by_key(AStarPoint::f_score);
    }

    None
}
