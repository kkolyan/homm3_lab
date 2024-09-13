use std::fmt;
use std::fmt::{Display, Pointer, Write};
use std::marker::PhantomData;

pub trait TableSource {
    fn get_size(&self) -> (usize, usize);

    fn read_value(&self, cell: (usize, usize), target: &mut dyn Write) -> fmt::Result;
}

#[derive(Default)]
pub struct ListTable<'a, T> {
    pub columns: Vec<(String, Box<dyn Fn(&T, &mut dyn Write) -> fmt::Result + 'a>)>,
    pub rows: Vec<T>,
}

impl <'a, T>  ListTable<'a, T> {
    pub fn add_column<'b>(&'b mut self, name: impl Display, renderer: impl Fn(&T, &mut dyn Write) -> fmt::Result + 'a) where 'a: 'b {
        self.columns.push((format!("{}", name), Box::new(renderer)));
    }

    pub fn add_row(&mut self, row: T) {
        self.rows.push(row);
    }
}

impl <'a, T> TableSource for ListTable<'a, T> {
    fn get_size(&self) -> (usize, usize) {
        (self.columns.len(), self.rows.len() + 1)
    }

    fn read_value(&self, (col, row): (usize, usize), target: &mut dyn Write) -> fmt::Result {
        let (column_name, column_renderer) = &self.columns[col];
        if row == 0 {
            write!(target, "{}", column_name.as_str())
        } else {
            let row = &self.rows[row - 1];
            column_renderer(row, target)
        }
    }
}

pub fn format_tsv<T: TableSource>(source: &T) -> Result<String, fmt::Error> {
    let mut s = String::new();

    let (cols, rows) = source.get_size();

    for col in 0..cols {
        source.read_value((col, 0), &mut s)?;
        if col < cols - 1 {
            s.push('\t');
        } else {
            s.push('\n');
        }
    }

    for row in 1..rows {
        for col in 0..cols {
            source.read_value((col, row), &mut s)?;
            if col < cols - 1 {
                s.push('\t');
            } else {
                s.push('\n');
            }
        }
    }
    Ok(s)
}