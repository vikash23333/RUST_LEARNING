fn main(){
    //Collection

    //Vector
    // let v= vec![1,2,3];   
    let mut v: Vec<i32>= Vec::new(); 

    //insert vec element
    v.push(1);  
    v.push(2);  
    v.push(25);  
    v.push(12);  
    v.push(25);  
    //access vector element:-
    let val = &v[1];//M1:-  problem of this we can specifiy a invelid index. so it will throw error on run time
    println!("val:- {}",val); 
    match v.get(10) {
        Some(x)=>println!("Value of index 10:- {}",x),
        None => println!("No Data found at index:-10")
    };
    print!("BEFORE ADDING IN LOOP:- ");
    for i in &v {
        print!("{} ", i);
    }

    println!();
    for item in &mut v {
        *item+=20;
    }


    print!("AFTER ADDING IN LOOP:- ");
    for i in &v {
        print!("{} ", i);
    }
    println!();

    //Next Exmaple:- 
    println!("Next Exmaple:- SpreadSheetCell");

    let row = [
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(11.1),
        SpreadSheetCell::Text(String::from("SHEET"))
    ];
    let x =&row[1];
    match  x{
        SpreadSheetCell::Int(i)=> println!("Type Interger is present at index:- {}",i),
        _=>println!("Not and Interger")
    }
    
}

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

