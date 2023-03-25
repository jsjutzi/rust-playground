use std::io::Write;
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T
}

// For any Pair<T>, allow new method to be called in fashing of Pair::new(x, y)
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self{x, y}
    }
}
// For any Pair<T> where T is any type that implements the Display & PartialOrd traits, allow cmp_display to be called in the fashion of [type].cmp_display()
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Generic Shared Functionality
fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn main() {
    let collection:Vec<i32> = vec![3, 55, 2, 23, 555, 3];
    let largest = find_largest(&collection);
    println!("{}", largest);

    let new_pair = Pair {x: 4, y: 2};
    let block_pair = Pair {
        x: vec!["hey there"],
        y: vec!["not today"]
    };

    new_pair.cmp_display();
    let test = Pair::new(4, 3);


}