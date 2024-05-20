enum Color {
    Red, Blue, Green , Yellow,
}

fn display_color(my_color: Color) {
    match my_color {
            Color::Red => println!("red"),
            Color::Blue => println!("blue"),
            Color::Green => println!("green"),
            Color::Yellow => println!("yellow"),
    }
}
fn main() {

        display_color(Color::Red);
        
}