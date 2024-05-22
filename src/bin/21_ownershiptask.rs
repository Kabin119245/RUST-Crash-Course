struct GroceryItem {
    quantity: i32,
    id : i32,
}

fn print_quantity(item: &GroceryItem) {
    println!("quantity is: {:?}", item.quantity);
}

fn print_id(item: &GroceryItem) {
    println!("ID is: {:?}", item.id);
}


fn main() {

    let gitem = GroceryItem {
        quantity: 20,
        id: 34,
    };

    print_quantity(&gitem);
    print_id(&gitem);


}