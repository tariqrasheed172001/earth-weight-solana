mod utils;
use utils::{calc::*, even_odd::*};

fn main() {
    let mut a = 10;    
    a = 12;
 
    println!("weight of {} is {}", 2, calculate_weight(5.0));
    
    let inp = Input {
        a: 20,
        b: 30
    };
    
    let pq = utils::sum(inp);
    
    println!("{}", pq);
}

fn calculate_weight(m: f32) -> f32 {
    m * 9.81
}