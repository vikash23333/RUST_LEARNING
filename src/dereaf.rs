use std::ops::Deref;

fn main(){
    // let x = 5;
    // let y = &x;
    // assert_eq!(5,x);
    // assert_eq!(5,*y);


    
    // let x = 5;
    let y = MyCustomBox::new("RUST");
    // assert_eq!(5,x);
    // assert_eq!(5,*y);//Behaviour of *y-> *(y.deref())
    hello(&y);
}

fn hello(x:&str) {
    println!("HELLO:- {}",x);
}


//Dreaf trait
struct MyCustomBox<T>(T);

impl <T> MyCustomBox<T> {
    fn new(x:T) ->MyCustomBox<T>{
        MyCustomBox(x)
    }
}

//Implement Dreaf Trait
impl <T> Deref for MyCustomBox<T> {
    type Target=T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}