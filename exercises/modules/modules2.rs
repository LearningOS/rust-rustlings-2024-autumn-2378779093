// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.


mod delicious_snacks {
    // Bring the PEAR constant from the fruits module into the scope of delicious_snacks and rename it to fruit
    pub use self::fruits::PEAR as fruit;
    // Bring the CUCUMBER constant from the veggies module into the scope of delicious_snacks and rename it to veggie
    pub use self::veggies::CUCUMBER as veggie;

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    // Access the fruit and veggie constants from the delicious_snacks module
    println!(
        "favorite snacks: {} and {}",
        crate::delicious_snacks::fruit,
        crate::delicious_snacks::veggie
    );
}