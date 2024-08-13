use anyhow::Result;

#[derive(Debug)]
struct Matrix<T> {
    date: Vec<T>,
    row: usize,
    col: usize,
}

fn main() -> Result<()> {
    println!("hello");
    Ok(())
}

//矩阵乘法
fn multiply<T>(a:&Matrix<T>,b:&Matrix<T>)->Result<Matrix<T>>{
    Ok(Matrix::new())
}