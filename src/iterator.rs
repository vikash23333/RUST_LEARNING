

fn main() {
    let v1 = vec![1,2,3];
    let item:Vec<_> = v1.iter().map(|x| x+1).collect();
    assert_eq!(item, vec![2,3,4]);


    let mut ctr = Counter::new();
    for _i in 0..10{
        let x =ctr.next();
        match x {
            Some(x)=>println!("VAL:- {:?}", x),
            None=>println!("VAL:- {:?}", x)
        }
        
    }
}

//Implement custom iterator
struct Counter{
    count:u32
}
impl Counter {
    fn new()->Counter {
        Counter{count:0}
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count<5 {
            self.count+=1;
            Some(self.count)
        }else{
            None
        }
    }
}





















#[derive(PartialEq,Debug)]
struct Shoe{
    size:u8,
    style:String
}

fn shoe_in_my_size(shoes:Vec<Shoe>, my_size:u8)->Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size==my_size).collect()
}


#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe{
            size:6,
            style:String::from("Sneaker")
        },
        Shoe{
            size:9,
            style:String::from("Sandal")
        },
        Shoe{
            size:6,
            style:String::from("Boot")
        },
        Shoe{
            size:7,
            style:String::from("XYZ")
        },
        Shoe{
            size:9,
            style:String::from("ABC")
        }
    ];
    let filter_my_size = shoe_in_my_size(shoes,6);
    assert_eq!(
        filter_my_size,
        vec![
            Shoe{
                size:6,
                style:String::from("Sneaker")
            },
            Shoe{
                size:6,
                style:String::from("Boot")
            },
        ]
    )
}