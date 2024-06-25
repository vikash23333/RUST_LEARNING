
//Life Time

// EX:- 1
// fn main(){
//     let r: &i32;
//     {
//         let x = 5;
//         r=&x;//Error:- x` does not live long enough. -borrowed value does not live long enough
//     }
//     println!("PRINT R:- {}",r);
// }

//EX:- 2
// fn main(){
//     let x = 5;
//     let r: &i32=&x; 
//     // let y = &r;
//     println!("PRINT R:- {}",r);
//     println!("PRINT x:- {}",x);
// }




// // EX:-3
// fn main(){
//     let s1 = String::from("VIKASH");
//     let s2 = String::from("kUMAR");
//     // let result = longest_error(&s1, &s2);
//     let result = longest_success(&s1, &s2);
//     println!("result:- {}",result)
// }
// fn longest_error(x:&str, y:&str)-> &str{//Throw:- Error:- missing lifetime specifier-  this function's return type contains a borrowed value,
//                                     //  but the signature does not say whether it is borrowed from `x` or `y`
//     if x.len()>y.len() {
//         x
//     }else{
//         y
//     }
// }

/*
TO FIX EX-3 Error:- 
    We use - generic life time specifier

    How to use lifetime notifier:- 
        lifetime notifier always start with ticks(') with some name/convension like:- 'a ya 'abc
    
    -Lets fix this longest_error function using lifetime notifier
    Ex:- &i32 lets use lifetime in this example
    --- &'a i32  -> a referance wirh explicit lifetime
    --- &'a mut i32 -> a mutable referance wirh explicit lifetime
*/

use std::fmt::Display;

fn longest_success<'a>(x:&'a str, y:&'a str)-> &'a str{ 
    if x.len()>y.len() {
        x
    }else{
        y
    }
}
//EX:- 4

// fn main(){
//     let s1 = String::from("VIKASH");
//     {
//         let s2 = String::from("kUMAR");
//         let result = longest_success(&s1, &s2);
//         println!("result:- {}",result)
//     }
    
// }

//EX:- 5
// fn main(){
//     let s1 = String::from("VIKASH");
//     let result; 
//     {
//         let s2 = String::from("kUMAR");
//         result = longest_success(&s1, &s2);//s2 Throw:- Error:- missing lifetime specifier-  this function's 
//                                                 // return type contains a borrowed value,
//         //                                     //  but the signature does not say whether it is borrowed from `x` or `y`
//     }
//     println!("result:- {}",result);
// }

/*
NOTE:- Return type of a function should be lifetime of that passed params of a function.
    we can't create a variable inside that function and return it from that function because that variable 
    lifetime is only valif till that function only.
*/

// Ex:- 6
struct ImportentPart<'a>{
    part: &'a str
}
// fn main() {
//     let nvl = String::from("Call me, 7 year ago");
//     let first_sen = nvl.split(',').next().expect("NOT FOUND");
//     let i = ImportentPart{
//         part:first_sen
//     };
//     println!("i:- {}",i.part);
// }


/*
        LIFETIME ELISION RULE:-
    
    1. Each params that is referance gets its own lifetime params.
    2. If there is exactly one input lifetime parmas.that lifetime is assigned to the all output lifetime params.
    3. If there is multiple input lifetime params, but one of them is &self or &mut then the lifetime of self is assigned to all 
        output lifetime parmas.  
*/
impl <'a> ImportentPart<'a> {
    //Originally it should be like:- fn rtn_part(&'a self,a: &str)->&'a str{
    fn rtn_part(&self,a: &str)->&str{// rule-3 applied
        println!("anno:- {}",a);
        self.part
    }
}

//Example:- 7: Static lifetime

fn main() {
    let s1 = String::from("HELLO");
    let s2 = String::from("WORLDDDD");
    let result = print_with_a_annocement(&s1, &s2, "HIII");
    println!("result:-{}",result)
}

fn print_with_a_annocement<'a, T>(x: &'a str, y: &'a str, ann:T)-> &'a str 
    where T:Display
{
    println!("Annocement:- {}",ann);
    if x.len()>y.len() {
        x
    }else{
        y
    }
}
