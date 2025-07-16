// 此函数获取一个盒子的所有权并销毁它
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("正在销毁包含 {} 的盒子", boxed_i32);
}

// 此函数借用一个 i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("这个整数是：{}", borrowed_i32);
}

fn main() {
    // 在堆上创建一个装箱的 i32，在栈上创建一个 i32
    // 注意：数字可以添加任意下划线以提高可读性
    // 5_i32 与 5i32 相同
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // 借用盒子的数据。所有权未被获取，
    // 所以数据可以再次被借用。
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // 获取盒子内数据的引用
        let _ref_to_i32: &i32 = &boxed_i32;

        // 错误！
        // 当内部值在作用域内稍后被借用时，不能销毁 `boxed_i32`。
        // eat_box_i32(boxed_i32);
        // 修复：^ 注释掉此行

        // 尝试在内部值被销毁后借用 `_ref_to_i32`
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` 超出作用域，不再被借用。
    }

    // `boxed_i32` 现在可以将所有权交给 `eat_box_i32` 并被销毁
    eat_box_i32(boxed_i32);
}
