fn c_n_normal_f32(c: f64, n: u32) -> f64 {
    (0..n).fold(1.0, |acc, _| acc * c)
}

fn c_n_small_error_f32(c: f64, n: u32) -> f64 {
    if n == 1 {
        c
    } else {
        let a = n / 2;
        let b = n - a;

        c_n_small_error_f32(c, a) * c_n_small_error_f32(c, b)
    }
}

fn c_n_normal_f64(c: f64, n: u32) -> f64 {
    (0..n).fold(1.0, |acc, _| acc * c)
}

fn c_n_small_error_f64(c: f64, n: u32) -> f64 {
    if n == 1 {
        c
    } else {
        let a = n / 2;
        let b = n - a;

        c_n_small_error_f64(c, a) * c_n_small_error_f64(c, b)
    }
}

fn main() {
    let n = 100000;

    let c = 1.00001;
    println!("f32 - {} is more accurate than {}", c_n_small_error_f32(c, n), c_n_normal_f32(c, n));

    let c = 1.00001;
    println!("f64 - {} is more accurate than {}", c_n_small_error_f64(c, n), c_n_normal_f64(c, n));
}
