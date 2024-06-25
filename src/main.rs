// Threads

use std::{thread, time::Duration};


//NOTE:- By default every program have one main thread.
fn main(){
    let handle= thread::spawn(|| {
        for i in 0..10 {
            println!("--------------{}------------SPOWN THREAD",i);
            thread::sleep(Duration::from_millis(1))
        }
    });
    //handle.join().unwrap(); // If i call here, then main thread will wait till spown thread start executing 
    for i in 0..5 {
        println!("Hii, No:-{} from MAIN thread",i);
        thread::sleep(Duration::from_millis(1))
    }
    //PROBLEM: IF MAIN THREAD END ITS MEANS SPOWN THREAD WILL ALSO END NOMATTER EXECUTING OR NOT
    //Solution:-
    handle.join().unwrap(); // After main thread stop then spown thread will finish rest of his work. 


}