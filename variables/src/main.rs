use std;
fn main() {
    const ONR_HOUR: usize = 1000 * 60 * 60;
    std::println!("{}", ONR_HOUR);
    let mut x = 5;
    std::println!("{}", x);
    x = 6;
    std::println!("{}", x);

    let y = 1;
    let y = y + 2;
    std::println!("{}", y);
    {
        let y = y * y;
        std::println!("{}", y);
    }
    std::println!("{}", y);

    let sum = 5 + 8;
    let diff = 5.0 - 8.1;
    let tup = (500, 1.1, 1, 'c', "chen");
    let (_, _, _, _, s) = tup;
    std::println!("{}", s);
    std::println!("{}", tup.2);
    std::println!("{}", tup.0);

    let arr = [10, 20, 30, 40, 50];
    loop {
        std::println!("请输入一个索引");
        let mut index = String::new();
        std::io::stdin().read_line(&mut index).expect("读取行失败");
        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        std::println!("{}", arr[index]);
        break;
    }
    another_function(32);
    test1();
    test2();
    let branch = if -1>0 {"x"} else {"y"};
    std::println!("{}", branch);

}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}


// 作用域
// 不同类型的数字不能运算
// 数组和元祖是固定长度
// 表达式和语句的区别
fn test1 () -> isize {5}
fn test2 () {5;}