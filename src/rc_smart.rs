//Reference Smart pointer


/*
    SMART POINTER DIFFERENCE:-
    
    1. Rc<T> enable multiple owner of the same data. Box<T> and RefCell<T> have single owner.
    2.Box<T> allow immutable anf mutable borrows checked at compile time.
      Rc<T > allowed only immutable borrows checked at compile time.
      Refcell<T> allow immutable anf mutable borrows checked at run time.
    3. Becaue RefCell<T> allow mutanle borrow check at run time, you can mute the value inside 
        the ref cell.even when the ref cell is immutable.
    





*/
 

use std::rc::Rc;


use crate::List::{Cons,Nil};
fn main(){
    let a  = Rc::new(Cons(2, Rc::new(Cons(4, Rc::new(Nil)))));
    println!("FIST REF COUNT:- {}",Rc::strong_count(&a));
    let b = Cons(9, Rc::clone(&a));//In this clone does not deep copy the data instead it increases the referance count.
    println!("2nd REF COUNT:- {}",Rc::strong_count(&a));
    let c = Cons(9, Rc::clone(&a));
    println!("3rd REF COUNT:- {}",Rc::strong_count(&a));
    {
        let d = Cons(9, Rc::clone(&a));
        println!("4th REF COUNT:- {}",Rc::strong_count(&a));
    }
    println!("last REF COUNT:- {}",Rc::strong_count(&a));

}

enum List{
    Cons(i32, Rc<List>),
    Nil
}