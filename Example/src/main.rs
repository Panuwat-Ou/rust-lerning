fn main() {
    let weather = "rainy";
    if weather == "sunny" {
        println!("It's a sunny day!");
    } else if weather == "rainy" {
        println!("It's a rainy day!");
    } else if weather == "stromy" {
        println!("Weather is stromy!!");
    } else {
        println!("Weather is unknown!");
    }

    let enemy = "dragon";
    match enemy {
        "goblin" => println!("A goblin appears!"),
        "troll" => println!("A troll appears!"),
        "dragon" => println!("A dragon appears!"),
        "skeleton" => println!("A skeleton appears!"),
        _ => println!("An unknown enemy appears!"),
    }
    let mut wood = 0;
    loop {
        if wood < 10 {
            println!("Gathering wood...{}", wood);
            wood += 1;
        } else {
            println!("Wood limit reached! You have {} wood.", wood);
            break;
        }
    }
}
