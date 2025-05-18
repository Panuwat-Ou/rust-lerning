fn main() {
    let enemy = "slime";
    match enemy {
        "slime" => println!("You encounter a slime!"),
        "goblin" => println!("You encounter a goblin!"),
        "dragon" => println!("You encounter a dragon!"),
        _ => println!("Unknown enemy!"),
        
    }
}
