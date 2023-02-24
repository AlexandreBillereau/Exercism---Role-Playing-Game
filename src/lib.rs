pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        let op = match self.health {
            0 => Some(Player {
                health: 100,
                mana: {
                    if self.level >= 10 {
                        Some(100)
                    } else {
                        None
                    }
                },
                level: self.level,
            }),
            _ => None,
        };

        return op;
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.level {
            0..=9 => {
                if mana_cost > self.health {
                    self.health = 0
                } else {
                    self.health -= mana_cost;
                }
                return 0;
            }
            _ => match self.mana {
                Some(mana) if mana < mana_cost => 0,
                Some(ref mut mana) => {
                    *mana -= mana_cost;
                    mana_cost * 2
                }
                None => 0,
            },
        }
    }
}
