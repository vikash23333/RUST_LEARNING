// Threads

use std::thread;//, time::Duration
// use std::sync::mpsc;
// use std::rc::Rc;
// use std::time::Duration;
use std::sync::{Arc,Mutex};

/*
NOTE:- By default every program have one main thread.
NOTE:- Refcell smart point allow you to mutate a value taht inside a Rc smart point. Same as Mutext smart pointer allow you to mutate
        a value inside the Arc smart pointer.

NOTE:- Refcell smart pointer come with the risk of creating circular dependencies & the Mutex smart pointer come with the risk of 
       creating deadlock
*/
fn main(){
    /*
    EX:- 1

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
     */





    /*EX:- 2 
    let v = vec![1,2,3];
    let handler = thread::spawn(move || {
        println!("SPAWN THREAD:- {:?}",v);

    });
    */

    /*
    
        //EX:-3  -MESSAGE PASSING CONCURRENCY 
    
        In rust there is multiple producer of message but only one reciver of message
        use mpsc module for messaging from std libariry.


    let (tx,rx) = mpsc::channel();
    let tx2  = tx.clone();

    thread::spawn(move || {
        // M1
        // let message = String::from("HII");
        // tx.send(message).unwrap();

        //M2:
        let vals = vec![
            "A",
            "B",
            "C" 
        ];
        for item in vals {
            tx.send(item).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || { 
        let vals = vec![ 
            "X",
            "Y",
            "Z"
        ];
        for item in vals {
            tx2.send(item).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    //M1
    // let recived = rx.recv().unwrap();//try_recv
    // println!("RECIVER MESSAGE:- {}",recived);


    //M2
    for i in rx {
        println!("ITEM GOT FROM RX1:- {}",i)
    }

     */


    /* EX:- 4  -SHARING STATE CONCURRENCY: 
        :- Mutex & lock 
        Mutual Exclusion(Mutex):- It means that you have some piece of data and only one thread could
            access that piece of data at any given time.
            -To acchive this mutex we use locking system.
            -lock is a data structurethat keep track of which thread has exclusive to a piece of data.
            -Once a thread has accuire a lock on a perticular piece of data No other thread 
                can access that data.One a thread is done with that piece of data it can unlock the data 
                and allow other thread to have access to It. 

            2 ROLE SHOUD BE FOLLOWED WHEN USEING MUTEX:- 
                1. We have to accuire a lock before having access to data.
                2. We have to release that lock when we done with data.

            NOTE:- Lock release will be automatic done, when scope end
    
    */
    //EX-4.1
    let mutex = Mutex::new(5);
    {
        let mut num = mutex.lock().unwrap();
        *num = 10;
    }
    println!("MMUTEX:- {:?}",mutex);


    let counter = Arc::new(Mutex::new(0));
    //  
    let mut handles = vec![];

    for _ in 0..10 {
        let ctr= Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = ctr.lock().unwrap();
            *num +=1;
        });
        handles.push(handle);
    }
    for handle in handles  {
        handle.join().unwrap();
    }
    println!("RESULT- {}",*counter.lock().unwrap());

}