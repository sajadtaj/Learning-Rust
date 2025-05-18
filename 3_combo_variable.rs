//---------------------------------------------------
//                      Array
//---------------------------------------------------
// طول ارایه مشخص است و از قبل تعیین شده
// همه عناصر ارایه باید از یک نوع باشند

// let array_name: [Type; size] = [element0, element1, ...];

// let a:[i32;5] = [12,23,32,1,3];
// let data = [21,23,43,12,23,44,5,68]

// می شود به ان مقدار جدید داد
// let mut array =[1,23,3,44]
// array[3] = 12


fn main(){
    let month :[&str; 12] = [
            "فروردین", "اردیبهشت", "خرداد",
            "تیر",     "مرداد",    "شهریور",
            "مهر",     "آبان",     "آذر",
            "دی",      "بهمن",     "اسفند"
            ];

    let first= month[0];
    let three= month[3];
    let last= month[11];

    println!("{}",first);
    println!("{}",three);
    println!("{}",last);

    println!("{:?}",month);     // نمایش کل آرایه


    let [q,w,e,r,t,y,u,i,o,p,a,s]=month;
    println!("{} {} {} {} {} {} {} {} {} {} {} {} ",q,w,e,r,t,y,u,i,o,p,a,s);
    println!("{:?}",[q,w,e,r,t,y,u,i,o,p,a,s]);
    println!("------------");
    function_range();

    println!("------------");
    tuple_function()


}

//---------------------------------------------------
//                      Array RAnge
//---------------------------------------------------
// [valueType;arrayLength]



fn function_range() {
    let x =[0i32;7];

    println!("Array Range");
    println!("x is: {:?}", x);

    println!("my array is: {:?}", [10i8;10]);
    println!("My array is : {:?}",[1.0009f32;4]);
}

//---------------------------------------------------
//                      Tuple
//---------------------------------------------------
// اندازه ثابت
// میتواند مقادیر از انواع مختلف بگیرد

// let tuple_name: (Type0, Type1, ...) = (Value0, Value1, ...);


fn tuple_function(){
    let t1 :(i32,char,bool,f64);
    t1 =(33,'s',true,99.88);
    
    let _t2 = (21,22222222,"iran",-9.89,false);

    println!("Tuple");
    println!("{}",t1.2);
    println!("{:?}",t1);

    let tup = (1, true, "سلام", 9.99);

    let (x, y, v, z) = tup;  // set tup to -> x,y,v,z
    println!("x: {}, y: {}, v: {}, z: {}", x, y, v, z);

}
