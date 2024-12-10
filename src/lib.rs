pub mod template;

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Write;

// Use this file to add helper functions and additional modules.

#[derive(Clone)]
pub struct Matrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn from(rows: usize, cols: usize, data: Vec<T>) -> Matrix<T> {
        assert_eq!(data.len(), rows * cols);
        Matrix { cols, rows, data }
    }

    pub fn get(&self, row: i32, col: i32) -> Option<&T> {
        if (0..self.rows as i32).contains(&row) && (0..self.cols as i32).contains(&col) {
            let pos = row as usize * self.cols + col as usize;
            let chr = self.data.get(pos).expect("Checked");
            return Some(chr);
        }
        None
    }

    pub fn update(&mut self, row: i32, col: i32, chr: T) -> Option<T> {
        if (0..self.rows as i32).contains(&row) && (0..self.cols as i32).contains(&col) {
            let pos = row as usize * self.cols + col as usize;
            let old = self.data.splice(pos..=pos, [chr]).last();
            return old;
        }
        None
    }
}
impl<T: Display> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows {
            f.write_char('\n')?;
            for c in 0..self.cols {
                let value = self.get(r as i32, c as i32).expect("Checked");
                f.write_str(value.to_string().as_str()).unwrap();
            }
        }
        Ok(())
    }
}
