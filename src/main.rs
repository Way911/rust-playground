use ndarray::arr2;
use ndarray::arr3;
use ndarray::Array2;
use ndarray::Array3;

fn main() {
    let mut a3 = Array3::<f64>::zeros((3, 4, 5));
    a3[[0, 0, 0]] = 0.0;
    a3[[1, 1, 1]] = 1.0;
    a3[[2, 2, 2]] = 2.0;
    println!("The 3D array is {:?}", a3);

    let mut a2 = Array2::<f64>::zeros((3, 3));
    a2[[0, 0]] = 0.0;
    a2[[0, 1]] = 0.5;
    a2[[0, 2]] = -0.5;
    a2[[1, 0]] = 1.0;
    a2[[1, 1]] = 1.5;
    a2[[1, 2]] = -1.5;
    a2[[2, 0]] = 2.0;
    a2[[2, 1]] = 2.5;
    a2[[2, 2]] = -2.5;
    println!("The 2D array is {:?}", a2);

    let mut a2 = arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    a2[[2, 1]] = 10;
    println!("The 2D array is {:?}", a2);

    let mut a3 = arr3(&[[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[9, 0], [1, 2]]]);
    a3[[2, 1, 1]] = 10;
    println!("The 3D array is {:?}", a3);

    let a3 = arr3(&[[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[9, 0], [1, 2]]]);
    for row in a3.rows() {
        // Each row is a 1D array
        println!("row is {:?}", row);
    }
    for a2 in a3.outer_iter() {
        println!("2D array is {:?}", a2);
    }
}
