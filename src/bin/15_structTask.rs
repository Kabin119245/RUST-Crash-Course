enum Flavour {
    Sweet,
    Sour,
    Salty,
    Fruity,
}

struct Drink {
    flavour : Flavour,
    fluid_oz : f64,
}


fn print_drink(drink: Drink) {
        match drink.flavour {
            Flavour::Fruity => println!("flavour: fruity"),
            Flavour::Sweet => println!("flavour: sweet"),
            Flavour::Salty => println!("flavour: salty"),
            Flavour::Sour => println!("flavour: sour"),
        }

        println!("oz: {:?}", drink.fluid_oz);
}

fn main() {


    let sweet = Drink {
        flavour: Flavour::Sweet,
        fluid_oz: 6.0,
    };
    print_drink(sweet);

    let salty = Drink {
        flavour: Flavour::Salty,
        fluid_oz: 5.0,
    };
    print_drink(salty);

    let fruity = Drink {
        flavour: Flavour::Fruity,
        fluid_oz: 5.5,
    };
    print_drink(fruity);

    let sour = Drink {
        flavour: Flavour::Sour,
        fluid_oz: 5.5,
    };
    print_drink(sour);

}