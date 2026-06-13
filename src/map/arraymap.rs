pub fn create_map() -> [[&'static str; 5]; 5]{
    let mut map = [
        ["* ","* ","* ","* ","*"],
        ["* ","* ","* ","* ","*"],
        ["* ","* ","x ","* ","*"],
        ["* ","* ","* ","* ","*"],
        ["* ","* ","* ","* ","*"]
    ];
    return map;
}

pub fn print_map(map: &mut [[&str; 5]; 5]) {
    for row in map.iter() {
        for column in row.iter() {
            print!("{}", column);
        }
        println!();
    }
}

pub fn move_up(map: &mut [[&str; 5]; 5], row: &mut usize, column: &mut usize) {
    if *row != 0{
        map[*row][*column] = "* ";
        *row -= 1;
        map[*row][*column] = "x ";
    } else {println!("Can't move out of the map");}
}

pub fn move_down(map: &mut [[&str; 5]; 5], row: &mut usize, column: &mut usize) {
    if *row != 4{
        map[*row][*column] = "* ";
        *row += 1;
        map[*row][*column] = "x ";
    } else {println!("Can't move out of the map");}
}

pub fn move_right(map: &mut [[&str; 5]; 5], row: &mut usize, column: &mut usize) {
    if *column != 4{
        map[*row][*column] = "* ";
        *column += 1;
        map[*row][*column] = "x ";
    } else {println!("Can't move out of the map");}
}

pub fn move_left(map: &mut [[&str; 5]; 5], row: &mut usize, column: &mut usize) {
    if *column != 0{
        map[*row][*column] = "* ";
        *column -= 1;
        map[*row][*column] = "x ";
    } else {println!("Can't move out of the map");}
}

