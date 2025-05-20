#[derive(Debug)]
struct Product {
    name : String,
    price : u32,
    quantity: u32
}

impl Product{  //9
    fn total_price(&self)-> u32{   //10
        self.price * self.quantity
    }

    fn increase_quantity(&self, amount:u32)-> Self{ //11
        Self{
            name: self.name.clone(),
            price: self.price.clone(),
            quantity : dbg!(self.quantity + amount)
            
        }
        
    }

    fn new(name:String, price: u32, quantity: u32)-> Self{ //12
        Self{
            name,
            price,
            quantity
        }
    }
}

fn create_product(price:u32 , name:String)-> Product{ //4
    let p = Product{
        name,
        price,
        quantity:1
    };
    return p
}


fn main(){
    let mut p1 = Product{
        name : String::from("لپتاپ"),
        price:1200,
        quantity:45
    };

    p1.quantity = 43;

    let p2 = Product {  //5
        name : String::from("کیبورد"),
        ..p1
    };

    let p4 = Product::new(String::from("Mnitor"),23,10);
    let p3 =create_product(143, String::from("Mouse"));

    println!("My Product is {p1:?}"); //7
    println!("My Product is {p2:?}");
    println!("My Product is {p3:?}");

    // 8
    dbg!(&p1);
    dbg!(&p2);

    println!("{}",p1.total_price());
    // 11
    p1=p1.increase_quantity(54);

    dbg!(&p1);

    println!("{p4:?}"); //12

}
