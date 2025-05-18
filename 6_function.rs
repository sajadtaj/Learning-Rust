// ام توابع را به صورت snake case می‌نویسم. 
// نام توابع با خروف کوچک نوشته می شود
// فقط تابع اصلی صدا زده می شود
// نوشتن نوع هر پارامتر ورودی الزامی است.



fn main(){

    println!("-----------");
    println!(" simple 1 ");
    simple_1(34);
    
    println!("-----------");
    println!(" simple 2 ");
    let y = simple_2(34);
    println!("{}",y);

    println!("-----------");
    println!(" simple 3 ");
    let x = simple_3(34);
    println!("{}",x);

    println!("-----------");
    println!(" recursive_function ");
    recursive_function(10);

    println!("-----------");
    println!(" Practice ");
    println!( "{}",practice_1(3));
    println!("-----------");
    println!(" Practice ");
    println!( "{}",practice_2(0));
    // practice(10);

}

fn simple_1(number:i32){
    println!("number = {}",number);
}


//Statement
//دستوراتی هستند که اعمالی را انجام می‌دهند, بدون اینکه مقداری را برگردانند
//به محض اینکه ; می‌آید, آن دستور تبدیل به یک استیتمنت می‌شود.
//  تعریف متغیر استیتنمنت هست


//expression
// دستوراتی که خروجی دارند.
// بعد از علامت = باید یک اکسپرشن قرار بگیرد 
// مثلاً اعمال ریاضی همه اکسپرشن هستند
// همچنین فراخوانی یک تابع یا ماکرو هم یک اکسپرشن است.
// یک مقدار عددی تنها هم می‌تواند یک اکسپرشن باشد. به شرط اینکه بعد از آن ; قرار نگرفته باش


//---------------------------------------------------
//                     Return
//---------------------------------------------------

fn simple_2(number:i32)-> i32 {
    let result = number *2;
    return result;
}


fn simple_3(number:i32) ->i32{

    number* 4  // بدون ;
}


//---------------------------------------------------
//                     Recursive Function
//---------------------------------------------------
fn recursive_function(mut number:i32) {
    if number <1 {
        return;
    }
    println!("current number is : {}",number);
    number -=1;
    recursive_function(number);
}


//---------------------------------------------------
//                     Practice
//---------------------------------------------------


//۱-تابع فاکتوریل را به صورت بازگشتی بنویسید.

fn practice_1( number:i64)-> i64{
    if number>1{
        return number * practice_1(number -1)
    }
    else { return 1}
    
}


// ۲-تابع فاکتوریل را با استفاده از حلقه‌ی for بنویسید.
fn practice_2( number:i64)-> i64{
    let mut x:i64=1;

    for _i in 1..number+1{
        
        x =x* _i;
    }
    return x;
}
