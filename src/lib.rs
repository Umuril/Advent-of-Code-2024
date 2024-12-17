pub mod template;

use std::ops::{Add, Mul, Sub};

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Write;

use forward_ref::forward_ref_binop;

// Use this file to add helper functions and additional modules.

#[derive(Clone)]
pub struct Matrix<T> {
    pub rows: isize,
    pub cols: isize,
    pub data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn from(rows: usize, cols: usize, data: Vec<T>) -> Matrix<T> {
        assert_eq!(data.len(), rows * cols);
        Matrix {
            cols: cols as isize,
            rows: rows as isize,
            data,
        }
    }

    pub fn get(&self, point: &Point) -> Option<&T> {
        if point.0 < 0 || point.1 < 0 || point.0 >= self.rows || point.1 >= self.cols {
            return None;
        }

        let pos = point.0 * self.cols + point.1;

        let chr = self.data.get(pos as usize).expect("Checked");
        Some(chr)
    }

    pub fn update(&mut self, point: &Point, chr: T) -> Option<T> {
        if point.0 < 0 || point.1 < 0 || point.0 >= self.rows || point.1 >= self.cols {
            return None;
        }

        let pos = (point.0 * self.cols + point.1) as usize;
        let old = self.data.splice(pos..=pos, [chr]).last();
        old
    }

    pub fn as_points<'a>(&self) -> Box<dyn Iterator<Item = Point> + 'a> {
        let rows = self.rows;
        let cols = self.cols;
        Box::from((0..rows).flat_map(move |x| (0..cols).map(move |y| Point(x, y))))
    }
}

impl<T: Display> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.rows {
            f.write_char('\n')?;
            for y in 0..self.cols {
                let value = self.get(&Point(x, y)).expect("Checked");
                f.write_str(value.to_string().as_str()).unwrap();
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point(pub isize, pub isize);

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

forward_ref_binop!(impl Add, add for Point, Point);

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

forward_ref_binop!(impl Sub, sub for Point, Point);

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, factor: isize) -> Self::Output {
        Self(self.0 * factor, self.1 * factor)
    }
}
