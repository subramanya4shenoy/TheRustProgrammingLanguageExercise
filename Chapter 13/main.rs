use std::thread;

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    
    fn giveaway(&self, user_preferences: Option<ShirtColor>) -> ShirtColor{
        user_preferences.unwrap_or_else(||self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor{
        let mut num_red: i32 = 0;
        let mut num_blue: i32 = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }

        if num_blue > num_red {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }
}

fn main() {
    let store: Inventory = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue
        ]
    };

    let user_pref1: Option<ShirtColor> = Some(ShirtColor::Red);
    let giveaway1: ShirtColor = store.giveaway(user_pref1);
    println!("User with pref {:?} get a {:?} t-shirt", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2: ShirtColor = store.giveaway(user_pref2);
    println!("User with pref2 {:?} get a {:?} t-shirt", user_pref2, giveaway2);


    // closure ownership behaviours
    let list = vec![1,2,3];
    let only_borrow = || println!("only borrow {:?}", list);
    only_borrow();

    let mut list = vec![1,2,4];
    let mut borrow_mutably = || list.push(23);
    borrow_mutably();
    println!("borrow mutably {:?}", list);

    thread::spawn(move || {
        println!("from thread: {:?}", list)
    }).join().unwrap();
}
