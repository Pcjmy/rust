mod mod1 {
    pub const MESSAGE: &str = "Hello World!";

    pub mod mod2 {
        pub const MESSAGE: &str = "Hello World!";
    }
}

fn main() {
    println!("{}", mod1::mod2::MESSAGE);
}
