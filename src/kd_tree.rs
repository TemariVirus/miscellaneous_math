use crate::{point::Point, Scalar};
use std::cmp::Ordering::*;

/// A static k-d tree structure I came up with myself to find closest colors before learning about k-d trees.
/// Ported from C#, and generalised to k dimensions (original was 3D only).
/// Reading up on Wikipedia makes me unsure about the corectness of my nearest neighbour search.
/// <br>https://en.wikipedia.org/wiki/K-d_tree#Nearest_neighbour_search
pub struct StaticKDTree<'a, T>
where
    T: 'a + Scalar,
{
    points: &'a [Point<T>],
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    mins: Vec<T>,
    maxes: Vec<T>,
}

impl<'a, T> StaticKDTree<'a, T>
where
    T: 'a + Scalar,
{
    pub fn new(points: &'a mut [Point<T>]) -> Result<Self, &str> {
        if points.is_empty() {
            return Err("Cannot create a k-d tree with no points");
        }

        self::axis_quicksort(points, 0);
        Ok(Self::new_with_axis(points, 0))
    }

    fn new_with_axis(points: &'a [Point<T>], axis: usize) -> Self {
        let len = points.len();
        let dims = points[0].len();
        let mut mins = vec![T::max_value(); dims];
        let mut maxes = vec![T::min_value(); dims];
        for i in 0..len {
            let point = &points[i];
            for j in 0..dims {
                let coord = point.get(j);
                if coord < mins[i] {
                    mins[i] = coord;
                }
                if coord > maxes[i] {
                    maxes[i] = coord;
                }
            }
        }

        if len <= 1 {
            return StaticKDTree {
                points,
                left: None,
                right: None,
                mins,
                maxes,
            };
        }

        let axis = (axis + 1) % dims;
        let left = Self::new_with_axis(&points[..len / 2], axis);
        let right = Self::new_with_axis(&points[len / 2..], axis);
        StaticKDTree {
            points,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            mins,
            maxes,
        }
    }

    pub fn size(&self) -> usize {
        self.points.len()
    }

    pub fn nearest_neighbour(&self, point: &Point<T>) -> (Point<T>, T) {
        if self.points.len() == 1 {
            let distance = self.points[0].distance_squared(point);
            return (self.points[0].clone(), distance);
        }

        let left = self.left.as_ref().unwrap().as_ref();
        let right = self.right.as_ref().unwrap().as_ref();
        let (first, second) = if left.distance_squared_from_boundary(point)
            < right.distance_squared_from_boundary(point)
        {
            (left, right)
        } else {
            (right, left)
        };

        let (p1, d1) = first.nearest_neighbour(point);
        if second.distance_squared_from_boundary(point) >= d1 {
            return (p1, d1);
        }

        let (p2, d2) = second.nearest_neighbour(point);
        if d1 < d2 {
            (p1, d1)
        } else {
            (p2, d2)
        }
    }

    fn distance_squared_from_boundary(&self, point: &Point<T>) -> T {
        let mut distance = T::zero();
        for i in 0..point.len() {
            let d = point.get(i).clamp(self.mins[i], self.maxes[i]) - point.get(i);
            distance += d * d;
        }

        distance
    }
}

fn partition<T>(points: &mut [Point<T>], pivot: T, axis: usize) -> usize
where
    T: Scalar,
{
    let mut split = 0;
    for i in 0..points.len() {
        if points[i].get(axis) < pivot {
            points.swap(i, split);
            split += 1;
        }
    }

    split
}

fn axis_quicksort<T>(points: &mut [Point<T>], axis: usize)
where
    T: Scalar,
{
    if points.len() <= 1 {
        return;
    }

    let pivot = self::approximate_median(points, axis);
    let split = self::partition(points, pivot, axis);

    let axis = (axis + 1) % points[0].len();
    self::axis_quicksort(&mut points[..split], axis);
    self::axis_quicksort(&mut points[(split + 1)..], axis);
}

fn approximate_median<T>(points: &mut [Point<T>], axis: usize) -> T
where
    T: Scalar,
{
    let mid = points.len() / 2;
    let pivot = points[mid].get(axis);
    if points.len() <= 50 {
        return pivot;
    }

    let split = self::partition(points, pivot, axis);

    match split.cmp(&mid) {
        Less => self::approximate_median(&mut points[(split + 1)..], axis),
        Equal => pivot,
        Greater => self::approximate_median(&mut points[..(split - 1)], axis),
    }
}
