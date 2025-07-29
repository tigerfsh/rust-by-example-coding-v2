macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e; // 强制类型为无符号整数
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

fn main() {
    calculate! {
        eval 1 + 2 // 嘿嘿，`eval` 可不是 Rust 的关键字哦！
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
