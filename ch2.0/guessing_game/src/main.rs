use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("猜猜我是几：");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read");

        // let guess: u32 = guess.trim().parse().expect("你这是数字吗");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("你这是数字吗？");
                continue;
            }
        };

        // println!("神秘数字为：{}", secret_number);
        println!("你刚刚输入了：{guess}");
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("太大了"),
            Ordering::Less => println!("太小了"),
            Ordering::Equal => {
                println!("好耶！");
                break;
            }
        }
    }
}
