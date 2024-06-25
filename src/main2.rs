
/*
CALL INSIDE FUNCTION:

variable();
string_learn();
conditional();
loops();

let sentance = String::from("MY NAME IS VIKASH");
let first_word = get_first_word(sentance);
println!("first_word:- {}",first_word);

mutable();
update_string();

main_owner_ex5();
main_owner_ex4();
main_owner_ex3_m2();
main_owner_ex3_m1();
main_owner_ex2();
main_owner_ex1();
main_owner_ex6_2();


 */

// use std::fs;//f64::consts::PI, 

fn main() { 
    // let user = User{
    //     name:String::from("Vikash kumar"),
    //     age:24
    // };
    // print!("{} is {} year old",user.name, user.age);

    // let rect = Rect{
    //     wth:30,
    //     hth:30
    // };
    // println!("Area of rect is :- {}", rect.area());
    // let my_dir = Direction ::North;
    // let dir_with_name = Direction ::South(String::from("VIKASH"));
    // move_dir(my_dir);    
    // println!("dir_with_name:- {:?}",dir_with_name);

    // let radius = Shape::Circle(5.0);
    // let len = Shape::Square(5.0);
    // let rect = Shape::Rectangle(5.0, 10.0);

    // println!("Area of a circle: {}",calculate_area(radius));
    // println!("Area of a square: {}",calculate_area(len));
    // println!("Area of a Rectangle: {}",calculate_area(rect));


    //Error Handling: Result Enum
    // let res: Result<String, std::io::Error> = fs::read_to_string("Exmple.txt");
    // match res {
    //     Ok(content)=>{
    //         println!("Text Content: {}",content);
    //     }
    //     Err(err)=>{
    //         println!("error while accessing the file:- {}",err);
    //     }
    // }

    // //Option Enum
    // let s = String::from("Vikash");
    // let s_nth_index = s.chars().nth(5 );//here s_nth_index type is : Option<char>
    // match s_nth_index {
    //     Some(char)=>{
    //         println!("Char at index 5 : {}",char)
    //     }
    //     None => {
    //         println!("No Data found at inxex 5 ")
    //     }
    // }
    //
    // let s_val = String::from("Vikash Krr");
    // let _res1 =find_first_a_with_option_trn(s_val);
    // let s_val1 = String::from("Vikash Krr");
    // let _res2 =find_first_a_with_result_trn(s_val1);


    let hot = Temprature{degree_f:50.0};
    hot.show_temp();
    let cold = Temprature::frezing();
    cold.show_temp();

}


struct Temprature{
    degree_f :f64
}
impl  Temprature{
    fn frezing()-> Self{
        Self{degree_f:12.2}
    }
    fn show_temp(&self){
        println!("Current temp is:- {}",self.degree_f);
    }
}  





// fn find_first_a_with_option_trn(s:String)->Option<usize> {
//     for (i, char) in s.chars().enumerate() {
//         if char=='a' {
//             return  Some(i);
//         }
//     }
//     return  None;
// }

// fn find_first_a_with_result_trn(s:String)->Result<usize,String> {
//     for (i, char) in s.chars().enumerate() {
//         if char=='a' {
//             return  Ok(i);
//         }
//     }
//     return  Err(String::from("A not found"));
// }



// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64, f64),
// }

// fn calculate_area(shape:Shape)->f64{
//     //Patern matching
//     match shape {
//         Shape::Circle(radius)=> PI*radius*radius,
//         Shape::Square(len)=> len*len,
//         Shape::Rectangle(h,w )=>h*w
//     }
// }









//Enums
// #[derive(Debug)]
// enum Direction {
//     North,South(String), East, West
// }
// fn move_dir(dir:Direction){
//     println!("DIR:- {:?}",dir);
// }
// //Impl
// struct Rect{
//     wth:u32,
//     hth:u32
// }

// impl  Rect {
//     fn area(&self)-> u32{
//         return self.wth * self.hth;
//     }
// }



// struct User {
//     name:String,
//     age:u8, 
// }

//Example-6
// fn main_owner_ex6_1(){
//     let mut s1 = String::from("Hello");
//     let s2 = &mut s1;
//     let s3 = &s1; //throw error:- cannot borrow `s1` as immutable because it is also borrowed as mutable
//     println!("{}",s2);
// }
// fn main_owner_ex6_2(){
//     let mut s1 = String::from("Hello");
//     let s2 = &mut s1;
//     update_word( s2); //throw error:-cannot borrow `s1` as mutable more than once at a time - second mutable borrow occurs here
//     // println!("{}",s1); //here S1: is out of scope
//     println!("{}",s2);
// }
// fn update_word(word:&mut String){
//     word.push_str("World");
// }


// // Example -5 -> Mutable reference
// fn main_owner_ex5(){
//     let mut s1 = String::from("Hello");
//     update_str_1(&s1);  
//     update_str_2(&mut s1);
//     println!("{}",s1);
// }
// fn update_str_1(s2:&String){
//     println!("{}",s2)
//    // s2.push_str("World"); //s2:- throw error: to update i have to make this s2 type mutable referance: &mut
// }
// fn update_str_2(s2:&mut String){
//     s2.push_str("World");  
// }




// //Example -4
// fn main_owner_ex4(){
//     let s1 = String::from("Hello");
//     take_ownership_ex4(&s1); // this function is borrowing the s1. but can override
//     println!("{}",s1);//This will work because i am re-assigned the owner ship to s1;
// }
// fn take_ownership_ex4(s2:&String){
//     println!("{}",s2); 
// }





// // Ex-3:- Borrowing and referencing
// fn main_owner_ex3_m2(){ // this will work because i am passing the refenrence.
//     // here s1:- owner  & s2:- borrower
//     let  s1 = String::from("Hello");
//     let s2 = &s1; // borrowing concept
//     println!("{}",s1);
//     println!("{}",s2); 
// }
// fn main_owner_ex3_m1(){ // 
//     let  s1 = String::from("Hello");
//     let s2 = s1;
//    // println!("{}",s1); //throw error:- s1 owner scope is dead. means owner is mover to s2.
//     println!("{}",s2); 
// }




// // Ex-2
// fn main_owner_ex2(){
//     let mut s1 = String::from("Hello");
//     s1 = take_ownership_ex2(s1);
//     println!("{}",s1);//This will work because i am re-assigned the owner ship to s1;
// }
// fn take_ownership_ex2(s2:String)-> String{
//     println!("{}",s2);
//     return s2;
// }



// // Exmaple:- 1
// fn main_owner_ex1(){
//     let s1 = String::from("Hello");
//     take_ownership_ex1(s1);
//     // println!("{}",s1);//Throw Error:- borrow of moved value: `s2`;
// }
// fn take_ownership_ex1(s2:String){
//     println!("{}",s2);
// }














// fn update_string(){
//     let mut s = String::from("INITIAL");
//     println!("INITIAL BEFFOR ADD:- LEN:- {} CAPICITY: {}, POINTER :-{:p}",s.len(),s.capacity(), s.as_ptr());
//     for _i in 0..100 {
//         s.push_str("XXXXXXXXX");
//         println!("AFTER UPDATE:- LEN:- {} CAPICITY: {}, POINTER :-{:p}",s.len(),s.capacity(), s.as_ptr());
// }
// }


// fn mutable(){
//     //throw error
//     // let x = 11;
//     // x = 70;
//     //making mutable
//     let mut x = 11;
//     println!("before x {}",x);
//     x = 70;
//     println!("x {}",x)
// }



// fn get_first_word(sentance:String)-> String{
//     //M1
//     // let mut first = String::from("");
//     //M2
//     let mut first = String::new();
//     //LOPPS: M1
//     // for item in sentance.chars(){
//     //     first.push_str(item.to_string().as_str());
//     //     if item==' '{
//     //         break;
//     //     }
//     // }
//     //LOOPS-M2
//     for (_i,c) in sentance.chars().enumerate() {
//         first.push_str(c.to_string().as_str());
//         if c==' '{
//             break;
//         }
//     }
//     return first;
// }

// fn loops(){
//     for i in 0..10 {
//         println!("{}",i);
//     }
// }

// fn conditional(){
//     let is_true = true;
//     if is_true{
//         println!("TRUE")
//     }else if !is_true{
//         println!("FALSE")
//     }
// }

// fn string_learn(){
//     let s1= String::from("VIKASH S1");
//     let s2 = "XVIKASH S2";
//     println!("s1: {} s2: {}",s1,s2);
//     //Print 0th index of s1:
//     let char0th = s1.chars().nth(0);
//     //M1:- SAFE WAY IS CHAR IS PRESENT ON INDEX OR NOT IT WONT THROW ERROR IT IS GETTING HANDELED
//     match char0th {
//         Some(c)=> println!("S1:- {}",c),
//         None => println!("S1:- No char found at index: 0")
//     }
//     let char10th = s2.chars().nth(10);
//     match char10th {
//         Some(c)=> println!("S2:- {}",c),
//         None => println!("S2:- No char found at index: 10")
//     }
//     //M2:- NOT THE SAFE WAY IF CHAR IS PRESENT AT INDEX IT WILL PRINT HE CHAR, ELSE RUN TIME ERROR:
//     let char1th = s1.chars().nth(1);
//     println!("M2:- {}",char1th.unwrap());
//     // let char111th = s1.chars().nth(111);
//     // println!("M2:- {}",char111th.unwrap()); // it will throw run time error
// }

// fn variable(){
//     let x:i32 =-5;
//     let y:u32 = 1000;
//     let z  = -100.11;
//     let a: &str  = "-100.11";
//     println!("variable: x: {} y: {} z: {}, a: {}",x,y,z,a);
// }
