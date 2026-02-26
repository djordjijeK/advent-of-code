#[derive(Clone, Copy)]
pub(crate) struct Coordinate {
    pub(crate) row: usize,
    pub(crate) column: usize
}

pub(crate) struct Grid<T> {
    rows: usize,
    columns: usize,
    data: Vec<T>
}


impl std::fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(value: (usize, usize)) -> Self {
        Self {
            row: value.0,
            column: value.1
        }
    }
}

impl<T> Grid<T> 
where
    T: Default + Clone
{
    pub(crate) fn new(rows: usize, columns: usize) -> Self {
        Self {
            rows: rows,
            columns: columns,
            data: vec![T::default(); rows * columns]
        }
    }


    pub(crate) fn rows(&self) -> usize {
        self.rows
    }


    pub(crate) fn columns(&self) -> usize {
        self.columns
    }


    pub(crate) fn in_bounds(&self, coordinate: Coordinate) -> bool {
        coordinate.row < self.rows && coordinate.column < self.columns
    }


    pub(crate) fn get_cell(&self, coordinate: Coordinate) -> Option<&T> {
        if !self.in_bounds(coordinate) {
            return None;
        }

        Some(& self.data[coordinate.row * self.columns + coordinate.column])
    }

        
    pub(crate) fn get_cell_mut(&mut self, coordinate: Coordinate) -> Option<&mut T> {
        if !self.in_bounds(coordinate) {
            return None;
        }

        Some(&mut self.data[coordinate.row * self.columns + coordinate.column])
    } 
}
