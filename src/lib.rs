mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn tabke_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
mod back_to_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast: &str) => Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_to_house::Breakfast::summer("Rye");   
    meal.toast = String::from("Wheat");             // 对于有pub的toast中，可以重新修改，但是私有的seasonal_fruit就不能修改了

    front_of_house::hosting::add_to_waitlist();
    // use 关键字使用
    hosting::add_to_waitlist();
    Front_of_house::hosting::add_to_waitlist();
}

use crate::front_of_house;
use self::front_of_house::hosting;   // Rust开发者们正尝试去掉self前缀，也许在不久可以避免使用它.
use crate::front_of_house as Front_of_house;
pub use self::front_of_house::hosting as Hosting;


