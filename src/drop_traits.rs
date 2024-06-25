fn main(){
    let a = CustomSmartPointer{
        data:String::from("FIRST A")
    };
    // a.drop()//ERROR-> Explicit destructor call is not allowed
    drop(a);//we can call drop like this
    
    let b = CustomSmartPointer{
        data:String::from("Second B")
    };
    println!("DROP CREATED");

    /*
        OUTPUT IN SEQUENCE:-
        DROP CREATED
        Second B
        FIRST A,

        vARIABKE WILL BE DROP AT REVERSE ORDER OF CREATION:- 
        1ST this msg get printed:- println!("DROP CREATED");
        Then,
        B WILL BE ROP THEN A.
    */
}


struct CustomSmartPointer{
    data:String
}

impl  Drop for  CustomSmartPointer {
    fn drop(&mut self) {
       println!("DROPED:- {}",self.data)
    }
}