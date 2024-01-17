// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let v = speed as u64;
    let vitesse  = (221 * v) as f64 ;   
    let val = if speed >= 1 && speed  <= 4 {

        vitesse as f64
    }else 
    {
        if speed >= 5 && speed <= 8
        {            
            (vitesse * 90 as f64) / 100  as f64      
        }
        else
        {             
             (vitesse * 77 as f64)  / 100 as f64         
        }
    };
    val

       
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let value = production_rate_per_hour(speed);
    let v = value / 60 as f64;
    v as u32
    }

    fn main()
    {
        let a = production_rate_per_hour(7);
        let b = production_rate_per_hour(9);
        println!("{}", a);
        println!("{}", b);
    }
