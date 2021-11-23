// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if (self.health == 0) {
            return Some(Player {
                health: 100,
                mana: match self.level {
                    0..=9 => None,
                    _ => Some(100),
                },
                level: self.level,
            })
        }
        None
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(m) if m < mana_cost => 0,
            Some(m) => {
                self.mana = std::cmp::max(Some(m - mana_cost), Some(0));
                mana_cost * 2
            },
            None => {
                self.health = if self.health > mana_cost {
                    self.health - mana_cost
                } else {
                    0
                };
                0
            }
        }
    }
}