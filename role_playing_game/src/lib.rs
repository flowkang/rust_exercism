// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::max;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health == 0 {
            Some(Player {
                health: 100,
                mana: if self.level >= 10 { Some(100) } else { None },
                level: self.level,
            })
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mana) => {
                if mana >= mana_cost {
                    self.mana = Some(mana - mana_cost);
                    mana_cost * 2
                } else {
                    0
                }
            }
            None => {
                self.health = if mana_cost > self.health { 0 } else {self.health - mana_cost};
                0
            },
        }
    }
}
