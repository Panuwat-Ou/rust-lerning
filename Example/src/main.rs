// fn main()
// {
//   let mut treasure: Vec<String>  = Vec::new();
//   treasure.push(String::from("gold"));
//   treasure.push(String::from("silver"));

//   treasure.pop();
//   treasure.remove(0);
  
//   let mut treasur2 = vec!["gold", "silver","gemstones"];

//   treasur2.len(); // 3 ปัจจุบันมี 3 อัน
//   treasur2.capacity(); // 3 จุได้เท่าไหร่
//   treasur2.pop();
//   treasur2.len(); // 2 ปัจจุบันมี 2 อัน
//   treasur2.capacity();// 3 จุได้เท่าไหร่

// }

// fn main()
// {  
//   //let mut item = vec!["gold", "silver","gemstones","emeralds"];
//   let mut item = vec!["gold"];
//   println!("item capacity is: {}", item.capacity());
//    item.push("diamond");
//   println!("item capacity is: {}", item.capacity());
//    item.push("platinum");
//   println!("item capacity is: {}", item.capacity());
//    item.push("gold2");

//   println!("item capacity is: {}", item.capacity());
//    item.push("silver2");
//   println!("item capacity is: {}", item.capacity());
//    item.push("gemstones2");
//     println!("item capacity is: {}", item.capacity());
//    item.pop();

//   println!("The treasure is: {:?}", item);
//   println!("item length is: {}", item.len());
//   println!("item capacity is: {}", item.capacity());
// }
fn main()
{  
  //let mut item = vec!["gold", "silver","gemstones","emeralds"];
  let mut item :Vec<String> = Vec::new();
  println!("item capacity start is: {}", item.capacity());
  let mut count = 0;
  while count < 20 {
    println!("push item : {}", format!("Gold{}", count+1));
    item.push(format!("Gold{}", count+1));
    println!("item length is: {}", item.len());
    println!("item capacity is: {}", item.capacity());
    println!("================================================");
    count += 1;
  }
}