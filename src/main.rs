mod mod1 {
    pub const MESSAGE: &str = "Hello World!";
    pub(self) const NUMBER: u32 = 16;

    pub(crate) enum CrateEnum {
        Item = 4
    }

    pub mod mod2 {
        pub const MESSAGE: &str = "Hello World!";

        pub fn say() {
            println!("{}", super::NUMBER);
        }
    }
}

fn main() {
    println!("{}", mod1::mod2::MESSAGE);
    println!("{}", mod1::CrateEnum::Item as u32);
    mod1::mod2::say();
}
