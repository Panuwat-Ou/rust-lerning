fn main()
{
  let treasures = ["Gold", "Silver", "Bronze", "Diamond","Emerald","Ruby"];
  let mut energy = 5;

  for treasure in treasures.iter(){
    if energy <= 0 { 
      println!("Out of energy");
      break;
    } else if treasure == &"Ruby" {
      println!("Found a Ruby! {}", treasure);
      break;
    } else {
      energy -= 1;
      println!("Found a treasure! {}, energy : {}", treasure, energy);
    }
  }
}