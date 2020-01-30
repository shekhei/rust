#![warn(unreachable_code)]

fn foo() -> Option<i32> { None }

pub fn bar(x: i32) {
    match foo() {
        Some(_) => return,
    }
    let _val = Box::new(x);
}

fn main() {}
