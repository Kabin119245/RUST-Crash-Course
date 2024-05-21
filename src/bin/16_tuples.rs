fn main() {


        let cord = (2,3);

        println!("{:?},{:?}", cord.0, cord.1);

        //destructring
        let (x,y) = (2,3);
        println!("{:?},{:?}", x, y);

        //different data types

        let (name, age) = ("KpOLi", 73);
        println!("{:?},{:?}", name, age);



        //BAD Example
        let favourites = ("red", 14, "TX", "pizza", "ELen show", "home");

        let state = favourites.2;
        let place = favourites.5;
        //so we create a structure



}