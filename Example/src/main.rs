use std::collections::HashMap;
// fn main()
// {  
//   let mut treasures = HashMap::new();
//   treasures.insert("gold", 100);
//   treasures.insert("silver", 50);

//   if let Some(gold) = treasures.get("gold") {
//       println!("Gold: {}", gold);
//   } else {
//       println!("No gold found.");
//   }
// }

fn main()
{  
  let mut treasures:HashMap<&str, i32>   = HashMap::new();
  treasures.insert("gold", 100);
  treasures.insert("silver", 50);


  treasures.get("Ruby").unwrap_or(&0); // Set Default Value when Key Not Found
  if let Some(gold) = treasures.get_mut("gold") {
    println!("Gold: {}", gold);
      *gold = *gold * 2 ;
      println!("Gold: {}", gold);
  } else {
      println!("No gold found.");
  }
}