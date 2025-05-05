use std::fmt::Display;

pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn cons<T>(x: T, list: List<T>) -> List<T> {
    List::Cons(x, Box::new(list))
}

pub fn create_list<T>(elements: Vec<T>) -> List<T> {
    let mut list = List::Nil;

    for element in elements.into_iter().rev() {
        list = cons(element, list);
    }
    list
}

pub fn display_cons_list<T: Display>(list: &List<T>) -> String {
    match list {
        List::Nil => format!("Nil"),
        List::Cons(value, next_list) => {
            format!("Cons({}, {})", value, display_cons_list(next_list))
        }
    }
}

pub fn sum_list(list: &List<i32>) -> i32 {
    match list {
        List::Nil => 0,
        List::Cons(value, next_list) => value + sum_list(next_list),
    }
}
