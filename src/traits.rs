use std::fmt::Display;

fn main(){
    let _tweet = Tweet{
        content:String::from("Hii There from tweet"),
        username:String::from("Vikash Kumar"),
        reply:String::from("Hey, Ho are U"),
        retweet:String::from("All good & U"),
    };
    let new_article = NewsArticle{
        author:String::from("Ajj Tak"),
        content:String::from("Died my mas**d"),
        headline:String::from("Some chutiya died"),
    };
    // println!("Tweet article: {}",tweet.summarize());
    // println!("News article: {}",new_article.summarize());
    // println!("Tweet Author: {}",new_article.get_author());
    // println!("News article Author: {}",new_article.get_author());

    //Pass Trails in function
    notify_t1_1(&new_article);
    notify_t1_2(&new_article);

    // Return traits from function:- 
    let traits = return_traits_struct();
    println!("traits func:- {}",traits.get_author())
}

//Traits:- traits are to Rust what interfaces are to Java or abstract classes are to C++.

pub trait Summary {
    //My implement this function:- required:- because it dont have default implementation
    fn summarize(&self) ->String;
    //Here get_author func have default implementation:- if is we dont impelent it myside our class\
    // Call this function then it will read default func
    fn get_author(&self) ->String{
     String::from("NO AUTHOR NAME")   
    }
}

pub struct  NewsArticle{
    pub author:String,
    pub headline:String,
    pub content:String,
}

pub struct  Tweet{
    pub username:String,
    pub content:String,
    pub reply:String,
    pub retweet:String,
}


//Implement Summary on Struct
impl Summary for NewsArticle {
    fn summarize(&self) ->String {
        format!("{} by {}",self.content,self.author)
    }
}

//Implement Summary on Struct
impl Summary for Tweet {
    fn summarize(&self) ->String {
        format!("{} by {}",self.content, self.username)
    }
    fn get_author(&self) ->String {
        format!("Author: {}",self.username)
    }
}

// Pass Tratils as params inside the function

//T-1.1
pub fn notify_t1_1(item: &impl Summary) {
    println!("notify: {}", item.summarize())
}

//T-1.2  Called:- Traits bound
pub fn notify_t1_2<T: Summary>(item: &T) { // This function is only limited to Summay Traits
    println!("notify: {}", item.summarize())
}


//T-2.1
pub fn notify_t2_1(item1: &impl Summary , item2: &impl Summary){
    println!("item1: {}, item2:- {}", item1.summarize(), item2.get_author())
}
//T-2.2
pub fn notify_t2_2<T:Summary+Display+PartialOrd+Clone>(item1: &T , item2: &T){//u can use multiple trails in one generic parmas:- Summary+Display
    println!("item1: {}, item2:- {}", item1.summarize(), item2.get_author())
}

//EX:-
// fn some_func<T,U>(_t:T, _u:U )->i32
//     where   T:Summary+Display,
//             U:Clone
// {
//     return 0;
// }

//Return a Traits:- Return only those which implement Summary traits:
fn return_traits_struct()-> impl Summary{
    // Tweet{
    //     content:String::from("Hii There from tweet"),
    //     username:String::from("Vikash Kumar"),
    //     reply:String::from("Hey, Ho are U"),
    //     retweet:String::from("All good & U"),
    // }
    NewsArticle{
        author:String::from("Ajj Tak"),
        content:String::from("Died my mas**d"),
        headline:String::from("Some chutiya died"),
    }
}


// Ex:- 

// struct Pair<T>{
//     x:T,
//     y:T,
// } 
