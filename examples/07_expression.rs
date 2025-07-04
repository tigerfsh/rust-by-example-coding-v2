fn main() {
    // A Rust program is (mostly) made up of a series of statements
    // here are a few kinds of statements in Rust. The most common two are declaring a variable binding, and using a ;
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;

    // Blocks are expressions too
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

// 表达式有返回值
