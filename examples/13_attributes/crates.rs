// This crate is a library
#![crate_type = "bin"]
// // The library is named "rary"
// #![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

fn main() {
    public_function();
    indirect_access();
}
