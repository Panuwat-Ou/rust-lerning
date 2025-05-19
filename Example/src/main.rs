struct Inventory<T> {
    item: T,
}
fn main()
{
    let gold_inventory = Inventory { item: 100 };
    let sword_inventory = Inventory { item: "Sword" };

    println!("Hold Gold: {}", gold_inventory.item);
    println!("Hold Sword: {}", sword_inventory.item);
    sword_inventory.display();
    gold_inventory.display();
}

trait DisplayItem {
    fn display(&self);
    
}

trait DisplayItemV2 {
    fn displayV2(&self);
    
}

impl <T: std::fmt::Debug> DisplayItem for Inventory<T> {
    fn display(&self) {
        println!("Inventory contains: {:?}", self.item);
    }
}
impl <T> DisplayItemV2 for Inventory<T> 
where T: std::fmt::Debug {
    fn displayV2(&self) {
        println!("Inventory contains: {:?}", self.item);
    }
}
