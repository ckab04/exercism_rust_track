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
       let new_player =  if self.health == 0{
       
            if self.level >= 10
        {
            Some(Player{
                health : 100,
                mana : Some(100),
                level: self.level,
            })

        }
        else {
            Some(Player{
                health: 100,
                mana : None,
                level : self.level
            } )           
        }
    }
    else{
        None
    };

        new_player

    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {


        if self.mana == None {
            let val = if self.health < mana_cost{
                self.health = 0;
                0
            }else {
                    self.health = self.health - mana_cost;
                    0
            };
            
            val
        }
        else{
            if self.mana.unwrap() < mana_cost
        {
           0
        }
            else{

                self.mana = Some(self.mana.unwrap() - mana_cost);
                2 * mana_cost        
            }

        }
        
    }
}
