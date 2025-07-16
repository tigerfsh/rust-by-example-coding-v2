fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    // Error! cannot move out of a type which implements the `Drop` trait
    //impl Drop for Person {
    //    fn drop(&mut self) {
    //        println!("Dropping the person struct {:?}", self)
    //    }
    //}
    // TODO ^ Try uncommenting these lines

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` 从 person 中被移出，但 `age` 只是被引用
    let Person { name, ref age } = person;

    println!("此人的年龄是 {}", age);

    println!("此人的姓名是 {}", name);

    // 错误！借用部分移动的值：`person` 发生部分移动
    //println!("person 结构体是 {:?}", person);

    // `person` 不能使用，但 `person.age` 可以使用，因为它没有被移动
    println!("从 person 结构体中获取的年龄是 {}", person.age);
}

/*
在这个例子中，我们将 age 变量存储在堆上以说明部分移动：删除上面代码中的 ref 会导致错误，因为 person.age 的所有权会被移动到变量 age。如果 Person.age 存储在栈上，就不需要 ref，因为 age 的定义会从 person.age 复制数据而不是移动它。）
*/
