

// شرط می تواند خارج از پرانتز باشد
// حتما بایذ عبارت شرط از نوع بولین باشد




fn main(){
    println!("conditon 1");
    con1();
    println!("******");

    println!("conditon 2");
    con2();
    println!("******");

    println!("conditon 3");
    con3();
    println!("******");

}

//---------------------------------------------------
//                      IF
//---------------------------------------------------

fn con1(){
    let a=10;
    if a> 9{
        println!("condition is correct -{}",a);
    } 
    else{
        println!("condition is wrong !!");
    }
}

//---------------------------------------------------
//                      else if
//---------------------------------------------------

fn con2(){
    let user = "Guest";

    if user == "Admin" {
        println!("hello {}",user);
    }

    else if user == "Guest" {
        println!("hello {}",user);
    }

    else if user == "Member" {
        println!("hello {}",user);
    }

    else {
        println!("user not detected !");
    }

}

//---------------------------------------------------
//                     if as expression
//---------------------------------------------------



fn con3(){
    let user = "iran" ;

    let a :i32 = if user == "iran"{
        44                     // a = 44
    }
    else{
        23                     // a = 23
    };
    println!("a = {}",a);
}

// بعد از else 
// ; 
// گذاشتیم چون جایست که مقدار دهی به متغیر تمام شده

// دربدنه برای مقدار دهی 
// ; 
//نگذاشته ایم 