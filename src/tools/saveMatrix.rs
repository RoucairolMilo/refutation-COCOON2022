use nalgebra::{DMatrix, Dynamic};
use std::fs::File;
use std::io::prelude::*;


pub fn save_matrix(name : &str, mat : DMatrix<f64>)-> std::io::Result<()>{
    let mut filename = "savedMatrix/".to_string();
    filename.push_str(name);
    filename.push_str(".txt");
    let mut file = File::create(filename)?;

    for i in 0..mat.column(0).len() {
        let mut s  = "".to_string();
        for j in 0..mat.column(0).len() {
            s.push_str(&*mat[(i, j)].to_string());
            s.push_str(", ");
        }
        s.push_str("\n");
        file.write(s.as_ref());
    }
    Ok(())
}