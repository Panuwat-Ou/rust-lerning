fn main()
{
  let map = String::from("Old map");

  let borrow_map = &map;
  let mut converted_map = borrow_map.to_string();
  converted_map.push_str(" to new map");
  println!("Converted map: {}", converted_map);
}