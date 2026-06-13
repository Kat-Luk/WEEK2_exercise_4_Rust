pub fn create_map() -> [[char; 5]; 5]{
    let map = [
        ['*', '*', '*', '*', '*'],
        ['*', '*', '*', '*', '*'],
        ['*', '*', 'x', '*', '*'],
        ['*', '*', '*', '*', '*'],
        ['*', '*', '*', '*', '*']
    ];
    return map;
} 

pub fn print_map(map: &[[char; 5]; 5]) {
    for row in map.iter() {
        for (i,column) in row.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{}", column);
        }
        println!();
    }
}

pub fn move_up(map: &mut [[char; 5]; 5], row: &mut usize, column: &mut usize) {
    if *row != 0 {
        map[*row][*column] = '*';
        *row -= 1;
        map[*row][*column] = 'x';
    } else {println!("Can't move out of the map");}
}

pub fn move_down(map: &mut [[char; 5]; 5], row: &mut usize, column: &mut usize) {
    if *row != 4{
        map[*row][*column] = '*';
        *row += 1;
        map[*row][*column] = 'x';
    } else {println!("Can't move out of the map");}
}

pub fn move_right(map: &mut [[char; 5]; 5], row: &mut usize, column: &mut usize) {
    if  *column != 4 {
        map[*row][*column] = '*';
        *column += 1;
        map[*row][*column] = 'x';
    } else {println!("Can't move out of the map");}
}

pub fn move_left(map: &mut [[char; 5]; 5], row: &mut usize, column: &mut usize) {
    if *column != 0{
        map[*row][*column] = '*';
        *column -= 1;
        map[*row][*column] = 'x';
    } else {println!("Can't move out of the map");}
}

