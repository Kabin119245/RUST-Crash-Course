enum Direction {
    Left,
    Right,
    Up,
}


fn main() {
    let go = Direction ::Left;
    match go {
        Direction::Left => println!("GO left"),
        Direction::Right => println!("GO Right"),
        Direction::Up => println!("GO Up"),


    }

}