// 这是一个名为 `say_hello` 的简单宏。
macro_rules! say_hello {
    // `()` 表示该宏不接受任何参数。
    () => {
        // 宏将展开成这个代码块的内容。
        println!("Hello!")
    };
}

fn main() {
    // 这个调用将展开成 `println!("Hello!")`
    say_hello!()
}
