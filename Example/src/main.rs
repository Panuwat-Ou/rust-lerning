// fn main() {
//     let treasure1 = "Gold coins";
//     let treasure2= "silver coins";
//     let Result = longest_treasure(treasure1, treasure2);
//     println!("the longest treasure is: {}", Result)
// }
// fn longest_treasure<'a> (x: &'a str, y: &'a str) -> &'a str{
//     if x.len() > y.len()
//     {
//         x
//     } else { y }
// }
fn longest_map<'a>(map1: &'a str ,map2: &'a str) -> &'a str{
    if map1.len() > map2.len()
    {
        map1
    } else { map2 }
}
fn main()
{
    let map1 = "Ancient Map of the sea";
    let map2 = "Map to hidden gold";
    let chosen_map = longest_map(map1,map2);
    println!("Crabby's longest map: {}",chosen_map);
}
