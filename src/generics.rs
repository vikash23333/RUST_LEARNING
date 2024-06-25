fn main(){
    //Generic
    let int_list = vec![1,6,2,4,1];
    let int_largest = get_largest(int_list);
    println!("int_largest:- {}",int_largest);


    let char_list:Vec<char> = vec!['q','r','t','c'];
    let char_largest = get_largest(char_list);
    println!("char_largest:- {}",char_largest);

}

// fn get_largest<T>(num: Vec<T>)->T{
//     let mut largest = num[0];
//     for item in num  {
//         /*
//              MAKING GENERIC I AM GETTING ERROR:-
//              '>' CANNOT APPLIED FOR GENERIC:
//              It's true, There is some type which is not compariable. thats why i am getting error
//              to solve this we have to use traits:- 
//              cgeck the
//          */
//         if item>largest { 
//             largest=item;
//         }
//     }
//     return largest;
// }

//Solution for above function:

fn get_largest<T:PartialOrd + Copy>(num: Vec<T>)->T{// Solution of '>'
    let mut largest = num[0];
    for item in num  { 
        if item>largest { 
            largest=item;
        }
    }

    return largest;
}