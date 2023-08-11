use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("猜一个数字");
    let select_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("请输入数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取行失败");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜的数字是: {guess}");
        match guess.cmp(&select_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


// match ... {} 这块的处理挺有意思，好像 js 的 switch+catch，很方便
// 定义的变量不能修改 除非用 mut 修饰。&变量表示引用，不会拷贝，&mut 可变引用
// 引用依赖要手动修改 toml 文件这个没有 npm 方便