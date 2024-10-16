#[test]
fn main() {
    let x = 1_000.000_1; // По умолчанию тип f64
    let y: f32 = 0.12;   // f32
    let z = 0.01_f64;    // f64

    assert_eq!(type_of(&x), "f64".to_string()); // Тип переменной x — f64
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
