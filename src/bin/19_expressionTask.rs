fn print_message(gt_100 : bool) {

            match gt_100 {
                true => println!("Its big"),
                false => println!("Its small")
            }

}

fn main() {

    //let value = 100;
    let value = 1000;


    let is_gt_100 = value > 100;

    print_message(is_gt_100);

}