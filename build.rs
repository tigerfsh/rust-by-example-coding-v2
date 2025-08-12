// https://course.rs/cargo/reference/build-script/intro.html

fn main() {
    if std::env::consts::OS == "linux" {
        println!("cargo:warning=Building for Linux");
    }
}
