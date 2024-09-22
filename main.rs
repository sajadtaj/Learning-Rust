// immutable |define in Function | just one time set value | can value from function
// mutable   |define in Function | every time can set value | can value from function
// const     | define in every where |just one time set value | can not value from function


const BUFFER_SIZE: i32 = 10; //const

fn main(){
    let _var1;           //immutable   متغیّر اول را مقدار دهی نکردیم. پس می•توانیم اینجا به آن مقدار بدهیم:
    let _var2 = 10;      //immutable
    let _var3: i32 = 20; //immutable
    _var1 = 50;          //mutable
    let mut _x = 10;     //mutable
    _x = 5;

    let _y = function();
    println!("{}",_y);
}

fn function() -> i32{
    println!("this function");
    return 10;
}


