fn main() {
    let warrior = Warrior {
        role: Melee,
        attack: Box::new(Attack1),
    };

    println!("{:?}", warrior.attack.base_attack());

    let rogue = Rogue {
        role: Melee,
        attack: Attack2,
    };

    println!("{:?}", rogue.attack.base_attack());
}

// Common parts
pub struct Melee;

pub trait Attack {
    fn base_attack(&self) -> usize;
}

// Case #1:
pub struct Warrior {
    pub role: Melee, // inherit from parent
    pub attack: Box<dyn Attack>,
}

// Case #2:
pub struct Rogue<T: Attack> {
    pub role: Melee, // inherit from parent
    pub attack: T,
}

// implementation of warrior's attack
struct Attack1;

impl Attack for Attack1 {
    fn base_attack(&self) -> usize {
        10
    }
}

// implementation of rogue's attack
struct Attack2;

impl Attack for Attack2 {
    fn base_attack(&self) -> usize {
        5
    }
}
