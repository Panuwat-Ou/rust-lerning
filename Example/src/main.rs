fn main()
{
    let mut player = warrior
    {
        name: String::from("Player"),
        health: 100,
    };

    player.take_damage(20);
    println!("{}'s health: {}", player.name, player.health);
    player.health(250);
    println!("{}'s health: {}", player.name, player.health);

    let current_state = player_state::Collecting(5);
    match current_state
    {
        player_state::Resting => println!("Player is resting"),
        player_state::Fighting => println!("Player is attacking"),
        player_state::Collecting(amount) => println!("Player is collecting {} items", amount),
        player_state::Defending => println!("Player is defending"),
    }

    let player_state_collecting = example_state::Collecting(5);
    player_state_collecting.describe();
    let player_state_fighting = example_state::Fighting;
    let player_state_defending = example_state::Defending;
    player_state_fighting.describe();
    player_state_defending.describe();
}

enum player_state
{
    Resting,
    Fighting,
    Collecting(u32),
    Defending,
}
struct warrior
{
    name: String,
    health: u8,
}

impl warrior
{
    fn health(&mut self, heal: u8)
    {
        if self.health.saturating_add(heal) >= 100
        {
            self.health = 100;
            return;
        }
    }

    fn take_damage(&mut self, damage: u8)
    {
        self.health = self.health.saturating_sub(damage);
    }
}

impl player_state
{
    fn describe(&self) 
    {
        match self
        {
            player_state::Resting => println!("Player is resting"),
            player_state::Fighting => println!("Player is attacking"),
            player_state::Collecting(amount) => println!("Player is collecting {} items", amount),
            player_state::Defending => println!("Player is defending"),
        }
    }
    
}

enum example_state
{
    Fighting,
    Collecting(u32),
    Defending,
}

impl example_state
{
    fn describe(&self) 
    {
        match self
        {
            example_state::Fighting => println!("Player is attacking"),
            example_state::Collecting(amount) => println!("Player is collecting {} items", amount),
            example_state::Defending => println!("Player is defending"),
        }
    }
    
}