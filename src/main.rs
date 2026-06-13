pub mod map;
use std::io;
use crate::map::arraymap::*;
fn main() {
    let mut row: usize = 2;
    let mut column: usize = 4;
    let mut map = create_map();
    print_map(&mut map); 
    loop {
        println!("| w) Move up | a) Move left | s) Move down | d) Move right | e) End program |");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Error reading input.");
        let choice = choice.trim();
        match choice {
            "e" => {
                println!("Ending the program.");
                break;
            }
            "a" => {
                move_left(&mut map, &mut row, &mut column);
                print_map(&mut map); 
            }
            "s" => {
                move_down(&mut map, &mut row, &mut column);
                print_map(&mut map); 
            }
            "d" => {
                move_right(&mut map, &mut row, &mut column);
                print_map(&mut map); 
            }
            "w" => {
                move_up(&mut map, &mut row, &mut column);
                print_map(&mut map); 
            }
            _ => {
                println!("Wrong input")
            }
        }
    }
}
