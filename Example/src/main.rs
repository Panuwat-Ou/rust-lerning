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