fn main() {
    let mut t: Vec<f64> = Vec::new();
    for i in 0..100 {
        t.push(f64::from(i));
    }
    let mut start = Vec::new();
    start.push(0.0);
    start.push(0.0);
    let result = rk4(start, t, dxdt);
    for i in result {
        for j in i {
            print!("Значение {}\n", j);
        }
    }
}

// Система дифференциальных уравнений
#[allow(unused_variables)]
fn dxdt(x: Vec<f64>, t: f64) -> Vec<f64> {
    vec![t * t, x[0] * x[0]]
}

// Функция для нахождения массива размера t.len()
// На вход подаются начальные значения value,
// Вектор значений независимой переменной t
// Функция, представляющая собой систему дифференциальных уравнений
// И возвращающая вычисленные правые части
// Функция использует метод Рунге-Кутта 4-го порядка точности
// Для численного решения системы
fn rk4(mut value: Vec<f64>, t: Vec<f64>, function: fn(Vec<f64>, f64) -> Vec<f64>) -> Vec<Vec<f64>> {
    let h: f64 = f64::from(t[1] - t[0]);
    let value_1 = value.clone();

    let mut result: Vec<Vec<f64>> = Vec::new();
    result.push(value.clone());

    for i in 0..t.len() {
        let mut k1: Vec<f64> = Vec::new();
        let mut value_2: Vec<f64> = Vec::new();

        for j in 0..value_1.len() {
            k1.push(function(value.clone(), t[i])[j] * h);
            value_2.push(value[j] + k1[j] / 2.0);
        }

        let mut k2: Vec<f64> = Vec::new();
        let mut value_3: Vec<f64> = Vec::new();
        for j in 0..value_1.len() {
            k2.push(function(value_2.clone(), t[i] + h / 2.0)[j] * h);
            value_3.push(value[j] + k2[j] / 2.0);
        }

        let mut k3: Vec<f64> = Vec::new();
        let mut value_4: Vec<f64> = Vec::new();
        for j in 0..value_1.len() {
            k3.push(function(value_3.clone(), t[i] + h / 2.0)[j] * h);
            value_4.push(value[j] + k3[j]);
        }

        let mut k4: Vec<f64> = Vec::new();
        for j in 0..value_1.len() {
            k4.push(function(value_4.clone(), t[i] + h)[j] * h);
        }

        let mut delta_value: Vec<f64> = Vec::new();
        for j in 0..value_1.len() {
            delta_value.push((k1[j] + 2.0 * k2[j] + 2.0 * k3[j] + k4[j]) / 6.0);
        }

        for j in 0..value_1.len() {
            value[j] = value[j] + delta_value[j];
        }
        result.push(value.clone());
    }
    result
}
