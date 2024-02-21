#[allow(dead_code, non_snake_case)]


pub mod sequences; 
use sequences::{reset, Fore, Back, Move};
use std::{thread, time};


fn main() {
    for i in 0..101 {
        Back::magenta();
        println!(" {}%", i);
        reset();
        Move::up(1);
        thread::sleep(time::Duration::from_millis(100));
    }
}



