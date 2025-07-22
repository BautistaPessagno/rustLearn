// definimos el modulo front_house
// adentro de un modulo podemos incluir mas modulos
mod front_house {
    //por default los modulos son privados por lo que no se puede acceder a ellos desde un padre se
    //usa pub para hacerlo publico
    pub mod hosting {
        //para que esto fucnione hace falta que la funcion tambien sea publica
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_over() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant1() {
    //absolute path
    crate::front_house::hosting::add_to_waitlist();

    //relative path
    front_house::hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_house::Breakfast::summer("Rye");
    //sa puede acceder a toast porque es publico, en cambio seasonal_fruit no lo es
    //y por lo tanto no se puede acceder
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast)
}
