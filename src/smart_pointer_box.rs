//Smart Pointer

/*

BOX-
    Heap Allocation:
        1.Large Data: If you have a large amount of data that you don't want to store on the stack 
        (which has limited space), you can use a Box to allocate it on the heap.
        2.Recursive Data Structures: Rust requires that recursive types have a known size at compile time, 
        which can be tricky for self-referential structures like linked lists or trees. Box can help by breaking 
        the recursive structure, allowing each node to be heap-allocated.

    Trait Objects:
        Dynamic Dispatch: Box can be used to create trait objects, enabling dynamic dispatch. 
        This is useful when you want to store a collection of different types that implement the same trait.
*/
//EX:- we can use Box in recursive enum
use List::{Cons, Nil};
fn main(){
    let box_val = Box::new(5);//Here, 5 is getting stored in the heap
    println!("box:- {}",box_val);

    let cons_list =Cons(5, Box::new(Cons(5, Box::new(Cons(9, Box::new(Cons(10, Box::new(Nil))))))));
}

enum List{
    Cons(i32, Box<List>),//Recursive enum
    Nil
}
/*
Cons:- Its is a data structure list like a Linkedlist:-
    it hold data and address of the second list and at the its contains address nil.
    cons(43, (cons 21, (const (11, nil))))
*/