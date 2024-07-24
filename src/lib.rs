#[no_mangle]
pub extern "C" fn matrix_multiply(a: *const f64, b: *const f64, c: *mut f64, n: usize) {
    let a = unsafe { std::slice::from_raw_parts(a, n * n) };
    let b = unsafe { std::slice::from_raw_parts(b, n * n) };
    let c = unsafe { std::slice::from_raw_parts_mut(c, n * n) };

    for i in 0..n {
        for j in 0..n {
            let mut sum = 0.0;
            for k in 0..n {
                sum += a[i * n + k] * b[k * n + j];
            }
            c[i * n + j] = sum;
        }
    }
}
