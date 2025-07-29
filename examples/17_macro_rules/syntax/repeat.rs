// `find_min!` will calculate the minimum of any number of arguments.
macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}
/*
结构解析
$($y:expr),+ 可以分解为：

$(...) - 表示"重复的模式"，括号内是重复的内容

$y:expr - 每个重复项是一个表达式（expr），命名为 $y

, - 重复项之间的分隔符（这里是逗号）

+ - 表示"至少重复一次"（one or more）
*/
