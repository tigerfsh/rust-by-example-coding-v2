// https://course.rs/cargo/reference/build-script/intro.html

use std;

fn main() {
    if std::env::consts::OS == "linux" {
        println!("cargo:warning=Building for Linux");
    }
}
