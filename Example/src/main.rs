use std::fmt::format;

fn main() {
    // Function calls
    greet();
    greet_user("Alice");
    let result = sum(5, 10);
    println!("Sum: {}", result);
    let result = greet_user_reply("Bob");
    println!("{}", result);
    let result = user_task("exercise", 30);
    println!("{}", result);
}

fn greet() {
    println!("Hello!");
}

fn greet_user(name: &str) {
    println!("Hello, {}!", name)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn greet_user_reply(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn user_task(task: &str,time :i32) -> String {
    format!("doing {} for {} minutes",task,time)
}
