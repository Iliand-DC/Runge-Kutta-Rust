fn main() {
    let mut t: Vec<i32> = Vec::new();
    for i in 0..100 {
        t.push(i);
    }
    let start = 0.0;
    let result = rk4(start, t, dxdt);
    for i in result {
        print!("{} ", i);
    }
}

#[allow(unused_variables)]
fn dxdt(x: f64, t: f64) -> f64 {
    t*t
}

fn rk4(start: f64, t: Vec<i32>, function: fn(f64, f64) -> f64) -> Vec<f64> {
    let h: f64 = f64::from(t[1] - t[0]);

    let mut result: Vec<f64> = Vec::new();
    result.push(start);

    for i in 0..t.len() {
        let k1 = function(result[i], f64::from(t[i]));
        let k2 = function((result[i] + h/2.0)*k1, f64::from(t[i])+h/2.0);
        let k3 = function((result[i] + h/2.0)*k2, f64::from(t[i])+h/2.0);
        let k4 = function((result[i] + h)*k3, f64::from(t[i])+h);

        result.push(result[i] + h/6.0*(k1 + 2.0*k2 + 2.0*k3 + k4));
    }
    result
}