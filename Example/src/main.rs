fn main()
{  
  let  treasures = vec![100, 200, 300, 400, 500];
  let double : Vec<i32>= treasures.iter().map(|x| x * 2 ).collect();
  println!("Treasures: {:?}", treasures);
  println!("Doubled Treasures: {:?}", double);
}