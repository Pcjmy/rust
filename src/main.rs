mod mod1 {
    pub const MESSAGE: &str = "Hello World!";

    pub(crate) enum CrateEnum {
        Item = 4
    }

    pub mod mod2 {
        pub const MESSAGE: &str = "Hello World!";
    }
}

fn main() {
    println!("{}", mod1::mod2::MESSAGE);
    println!("{}", mod1::CrateEnum::Item as u32);
}
