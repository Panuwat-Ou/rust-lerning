fn main() {
    let mut treasure = String::from("Gold Coin");
    let friend1 = &treasure;
    let friend2 = &treasure;
    println!("Friend 1 see:{}",friend1);
    println!("Friend 2 see:{}",friend2);

    let trusted_friend = &mut treasure;
    trusted_friend.push_str(" and Silver coin");
    println!("Trusted friend update:{}", trusted_friend);
}
