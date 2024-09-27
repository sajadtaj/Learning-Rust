fn main(){
    println!("Loop");
    // loop_test();          // this is a loop !!!!
    println!("-------");

    println!("Break");
    break_loop();
    println!("-------");

    println!("while");
    while_test();
    println!("-------");

    println!("For");
    for_test();
    println!("-------");

    println!("For on array");
    for_test_2();
    println!("-------");

    println!("Practice");
    for_test_3();
    println!("-------");

}

//---------------------------------------------------
//                     loop
//---------------------------------------------------
// استفاده از لوپ در برنامه های چند رشته ای و منتظر پاسخ بودن کاربرد دارد
// run for  ever until ctrl+c
fn loop_test(){

    loop{
        println!("run for  ever until ctrl+c");
    }

}


//---------------------------------------------------
//                     Break
//---------------------------------------------------

// set value for a by loop and break

fn break_loop(){
    let mut counter = 0;
    let a = loop{
        if counter == 6 {
            break counter;              // by break return counter
        }
        counter +=1;
    };

    println!("a = {}",a);
}

//---------------------------------------------------
//                    while
//---------------------------------------------------
// while condition {
//     // do something
// }

fn while_test(){
    let mut a =1;
    while a % 10 !=0 {
        println!("a = {}",a);
        a +=1;
    }
}

//---------------------------------------------------
//                    for
//---------------------------------------------------

// for counter in range {
//     // your code
// }
// نمی‌توانید مقدار شمارنده‌را داخل بدنه‌ی حلقه تغییر بدهید,


// range : m..n+1 -> from 'm' to 'n'

fn for_test(){
    for counter in 1..11{
        println!("counter is : {}",counter);
    }
}

//---------------------------------------------------
//                    for on Array
//---------------------------------------------------

fn for_test_2(){
    let a:[i32;6] = [1,22,-21,3,768,9999];

    for i in a.iter(){
        println!("i is : {}",i);
    }

}

//---------------------------------------------------
//                    for on Array
//---------------------------------------------------
fn for_test_3(){
    println!("");

    
    for i in 1..6{

        for _ in 1..i+1{
            print!("*");
        }
        println!("");
    }

    let mut counter = 5;
    for _ in 1..counter{
        for _ in 1..counter{
            print!("*");
        }
        counter -=1;
        println!("");

    }

    
}