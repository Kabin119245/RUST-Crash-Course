//Data management using tuples

fn coordinates() -> (i32, i32) {
    let (a,b) = (10,5);
    (a,b)
}
fn main() {
    //Destructuring the return value into two variables
    let (x,y) = coordinates();
    println!("{:?},{:?}", x, y);


    if y > 5 {
        println!(">5")
    } else if y < 5 {
        println!("<5")
    } else {
        println!("=5")
    }


}