use anyhow::{anyhow, Result};
use std::{
    fmt::{self, Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

use crate::Vector;

const NUM_THREADS: usize = 4;

pub struct Matrix<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

pub struct MsgInput<T> {
    // idx: usize,
    rol: Vec<T>,
    col: Vec<T>,
}

pub struct MsgOutput<T> {
    // idx: usize,
    val: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

//pretend this is a heavy operation,CPU intensive
fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Default + Copy,
{
    if a.len() != b.len() {
        return Err(anyhow!("Vector dimensions do not match"));
    }
    let mut sum = T::default();
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    Ok(sum)
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Default + Copy + Send + 'static,
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix dimensions do not match"));
    }

    //generate 4 threads to receive msg and do the dot product
    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let val = dot_product(Vector::new(msg.input.rol), Vector::new(msg.input.col))?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        // idx: msg.input.idx,
                        val,
                    }) {
                        eprintln!("Error: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let matrix_len = a.rows * b.cols;

    let mut data = Vec::with_capacity(matrix_len);
    let mut receivers = Vec::with_capacity(matrix_len);

    for i in 0..a.rows {
        for j in 0..b.cols {
            let row = Vector::new(a.data[i * a.cols..(i + 1) * a.cols].to_vec());
            let column_data = (0..b.rows)
                .map(|k| b.data[k * b.cols + j])
                .collect::<Vec<_>>();
            let column = Vector::new(column_data);
            let input = MsgInput::new(row.to_vec(), column.to_vec());
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[i % NUM_THREADS].send(msg) {
                eprintln!("Error: {:?}", e);
            }

            receivers.push(rx);

            for rx in receivers.drain(..) {
                let output = rx.recv()?;
                data.push(output.val);
            }
        }
    }

    Ok(Matrix {
        data,
        rows: a.rows,
        cols: b.cols,
    })
}

impl<T: Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Self {
            data: data.into(),
            rows,
            cols,
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: Display,
{
    // display a 2x3 as {1 2 3, 4 5 6}, 3x2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.rows {
            write!(f, "{}", if i == 0 { "{" } else { " " })?;
            for j in 0..self.cols {
                write!(
                    f,
                    "{}{}",
                    if j == 0 { "" } else { " " },
                    self.data[i * self.cols + j]
                )?;
            }
            write!(f, "{}", if i == self.rows - 1 { "}" } else { "," })?;
        }
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Matrix {{ data: {:?}, rows: {}, cols: {} }}",
            self.data, self.rows, self.cols
        )
    }
}

impl<T> MsgInput<T> {
    pub fn new(rol: Vec<T>, col: Vec<T>) -> Self {
        Self { rol, col }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        let a = Matrix::new(vec![1, 2, 3, 4], 2, 2);
        let b = Matrix::new(vec![5, 6, 7, 8], 2, 2);
        let c = multiply(&a, &b).unwrap();
        assert_eq!(format!("{}", c), "{19 22, 43 50}");
    }

    #[test]
    fn test_display() {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        assert_eq!(format!("{}", a), "{1 2 3, 4 5 6}");
    }
}
