use std::io;

fn main() {
    println!("猜数字, 请输入你的猜测：");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("读取失败");
    println!("你猜的数字是：{}", guess);
}
