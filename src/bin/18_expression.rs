enum Access {
    Admin, 
    Manager,
    User,
    Guest,
}

fn main() {

        //secret file: admins only

        let access_level = Access::Guest;

        let can_access_files = match access_level {
            Access::Admin => true,
            _ => false,
        };

        println!("can access: {:?}", can_access_files);

}