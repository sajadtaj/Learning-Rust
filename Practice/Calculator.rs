use std::io;

fn main() {
    println!("ماشین حساب ساده به زبان Rust");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut operator = String::new();

    // گرفتن عدد اول
    println!("عدد اول را وارد کنید:");
    io::stdin().read_line(&mut input1).expect("خطا در خواندن ورودی");
    let num1: f64 = match input1.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("ورودی نامعتبر است.");
            return;
        }
    };

    // گرفتن عملگر
    println!("عملگر را وارد کنید (+, -, *, /):");
    io::stdin().read_line(&mut operator).expect("خطا در خواندن ورودی");
    let operator = operator.trim();

    // گرفتن عدد دوم
    println!("عدد دوم را وارد کنید:");
    io::stdin().read_line(&mut input2).expect("خطا در خواندن ورودی");
    let num2: f64 = match input2.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("ورودی نامعتبر است.");
            return;
        }
    };

    // انجام محاسبه
    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("خطا: تقسیم بر صفر امکان‌پذیر نیست.");
                return;
            }
            num1 / num2
        },
        _ => {
            println!("عملگر نامعتبر است.");
            return;
        }
    };

    println!("نتیجه: {}", result);
}
