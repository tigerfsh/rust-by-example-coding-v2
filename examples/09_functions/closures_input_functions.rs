// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
fn call_me<F>(f: F)
where
    F: Fn(),
{
    f();
}

// Define a wrapper function satisfying the `Fn` bound
fn function() {
    println!("I'm a function!");
}

fn main() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure);
    call_me(function);
}
