
use std::{ops, fmt};

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        assert_eq!(row * col, values.len());
        Matrix {
            data: (*values).to_vec(),
            row: row,
            col: col,
        }
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
        Matrix{
            row: row,
            col: col,
            data:Vec::with_capacity(row*col),
        }
    }

    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.row, rhs.row);
        assert_eq!(self.col, rhs.col);
        Matrix {
            row:self.row,
            col:self.col,
            data: {
                let mut result:Vec<T> = Vec::new();
                for i in 0..self.row*self.col
                {
                    result.push(self.data[i]+rhs.data[i]);
                }
                result
            }
        }
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Matrix<T>) -> Self::Output {
        assert_eq!(self.row, rhs.row);
        assert_eq!(self.col, rhs.col);
        Matrix {
            row:self.row,
            col:self.col,
            data: {
                let mut result:Vec<T> = Vec::new();
                for i in 0..self.row*self.col
                {
                    result.push(self.data[i]+rhs.data[i]);
                }
                result
            }
        }
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.row, rhs.row);
        assert_eq!(self.col, rhs.col);
        Matrix {
            row:self.row,
            col:self.col,
            data: {
                let mut result:Vec<T> = Vec::new();
                for i in 0..self.row*self.col
                {
                    result.push(self.data[i]+rhs.data[i]);
                }
                result
            }
        }
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: &Self) -> Self::Output {
        assert_eq!(self.row, rhs.row);
        assert_eq!(self.col, rhs.col);
        Matrix {
            row:self.row,
            col:self.col,
            data: {
                let mut result:Vec<T> = Vec::new();
                for i in 0..self.row*self.col
                {
                    result.push(self.data[i]+rhs.data[i]);
                }
                result
            }
        }
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.row, rhs.row);
        assert_eq!(self.col, rhs.col);
        Matrix {
            row:self.row,
            col:self.col,
            data: {
                let mut result:Vec<T> = Vec::new();
                for i in 0..self.row*self.col
                {
                    result.push(self.data[i]-rhs.data[i]);
                }
                result
            }
        }
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        assert_eq!(self.row, rhs.row);
        assert_eq!(self.col, rhs.col);
        Matrix {
            row:self.row,
            col:self.col,
            data: {
                let mut result:Vec<T> = Vec::new();
                for i in 0..self.row*self.col
                {
                    result.push(self.data[i]-rhs.data[i]);
                }
                result
            }
        }
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.row, rhs.row);
        assert_eq!(self.col, rhs.col);
        Matrix {
            row:self.row,
            col:self.col,
            data: {
                let mut result:Vec<T> = Vec::new();
                for i in 0..self.row*self.col
                {
                    result.push(self.data[i]-rhs.data[i]);
                }
                result
            }
        }
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: &Self) -> Self::Output {
        assert_eq!(self.row, rhs.row);
        assert_eq!(self.col, rhs.col);
        Matrix {
            row:self.row,
            col:self.col,
            data: {
                let mut result:Vec<T> = Vec::new();
                for i in 0..self.row*self.col
                {
                    result.push(self.data[i]-rhs.data[i]);
                }
                result
            }
        }
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.col, rhs.row);
        Matrix {
            row:self.row,
            col:rhs.col,
            data: {
                let mut result:Vec<T> = Vec::new();
                for i in 0..self.row
                {
                    for j in 0..rhs.col
                    {
                        let mut temp:T = self.data[i*self.col]*rhs.data[j];
                        for n in 1..self.col
                        {
                            temp = temp+self.data[i*self.col+n]*rhs.data[j+n*rhs.col]
                        }
                        result.push(temp);
                    }

                }
                result
            }
        }
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        assert_eq!(self.col, rhs.row);
        Matrix {
            row:self.row,
            col:rhs.col,
            data: {
                let mut result:Vec<T> = Vec::new();
                for i in 0..self.row
                {
                    for j in 0..rhs.col
                    {
                        let mut temp:T = self.data[i*self.col]*rhs.data[j];
                        for n in 1..self.col
                        {
                            temp = temp+self.data[i*self.col+n]*rhs.data[j+n*rhs.col]
                        }
                        result.push(temp);
                    }

                }
                result
            }
        }
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.col, rhs.row);
        Matrix {
            row:self.row,
            col:rhs.col,
            data: {
                let mut result:Vec<T> = Vec::new();
                for i in 0..self.row
                {
                    for j in 0..rhs.col
                    {
                        let mut temp:T = self.data[i*self.col]*rhs.data[j];
                        for n in 1..self.col
                        {
                            temp = temp+self.data[i*self.col+n]*rhs.data[j+n*rhs.col]
                        }
                        result.push(temp);
                    }

                }
                result
            }
        }
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: &Self) -> Self::Output {
        assert_eq!(self.col, rhs.row);
        Matrix {
            row:self.row,
            col:rhs.col,
            data: {
                let mut result:Vec<T> = Vec::new();
                for i in 0..self.row
                {
                    for j in 0..rhs.col
                    {
                        let mut temp:T = self.data[i*self.col]*rhs.data[j];
                        for n in 1..self.col
                        {
                            temp = temp+self.data[i*self.col+n]*rhs.data[j+n*rhs.col]
                        }
                        result.push(temp);
                    }

                }
                result
            }
        }
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        assert_eq!(self.row*self.col,self.data.len());
        for i in 0..self.row
        {
            for j in 0..self.col
            {
                try!(write!(f, "{}", &self.data[i*self.col+j]));
                if j != self.col-1
                {
                    try!(write!(f, " "));
                }
            }
            try!(write!(f, "\n"));
        }
        write!(f, "", )
    }

}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test() {
        let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
        let y = Matrix::new(2, 3, &[1, 2, 3, 4, 5, 6]);
        assert_eq!(&x + &y - &y, x);
        assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
    }
}
