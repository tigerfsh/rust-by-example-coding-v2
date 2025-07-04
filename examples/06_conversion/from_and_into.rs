use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[derive(Debug)]
struct NumberV2 {
    value: i32,
}
impl Into<NumberV2> for i32 {
    fn into(self) -> NumberV2 {
        NumberV2 { value: self }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
    println!("value of number: {}", num.value);

    let int = 5;
    // Try removing the type annotation
    let num: NumberV2 = int.into();
    println!("My NumberV2 is {:?}", num);
}

/*
实现了From，则自动实现了Into；反之则不成立
*/
