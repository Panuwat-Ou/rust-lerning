fn main() {
    let x = 5;
    let y = 20.0;
    let a = 25;
    let b = 30.7;
    let sum = x + y as i32;
    let sum2 = a as f32 + b;
    let message = String::from("Hello, Panuwat {}");
    let msg = "Hello".to_string();
    let msg2 = format!("Hello, Panuwat {}, {}", sum, sum2);
    let msg3 = "hello_werotk";
    println!("{}", msg2);
}
