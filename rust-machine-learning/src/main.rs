use rust_machine_learning::matrix_multiply;

fn main() {
    let n = 2;
    let a = vec![1.0, 2.0, 3.0, 4.0];
    let b = vec![5.0, 6.0, 7.0, 8.0];
    let mut c = vec![0.0; n * n];
    
    matrix_multiply(a.as_ptr(), b.as_ptr(), c.as_mut_ptr(), n);

    println!("{:?}", c);
}