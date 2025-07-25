#![allow(clippy::assertions_on_constants, clippy::equatable_if_let, clippy::needless_if)]
#![warn(clippy::collapsible_if, clippy::collapsible_else_if)]

#[rustfmt::skip]
fn main() {
    let x = "hello";
    let y = "world";
    // Collapse `else { if .. }` to `else if ..`
    if x == "hello" {
        print!("Hello ");
    } else {
        if y == "world" {
            println!("world!")
        }
    }
    //~^^^^^ collapsible_else_if

    if x == "hello" {
        print!("Hello ");
    } else {
        if let Some(42) = Some(42) {
            println!("world!")
        }
    }
    //~^^^^^ collapsible_else_if

    if x == "hello" {
        print!("Hello ");
    } else {
        if y == "world" {
            println!("world")
        }
        else {
            println!("!")
        }
    }
    //~^^^^^^^^ collapsible_else_if

    if x == "hello" {
        print!("Hello ");
    } else {
        if let Some(42) = Some(42) {
            println!("world")
        }
        else {
            println!("!")
        }
    }
    //~^^^^^^^^ collapsible_else_if

    if let Some(42) = Some(42) {
        print!("Hello ");
    } else {
        if let Some(42) = Some(42) {
            println!("world")
        }
        else {
            println!("!")
        }
    }
    //~^^^^^^^^ collapsible_else_if

    if let Some(42) = Some(42) {
        print!("Hello ");
    } else {
        if x == "hello" {
            println!("world")
        }
        else {
            println!("!")
        }
    }
    //~^^^^^^^^ collapsible_else_if

    if let Some(42) = Some(42) {
        print!("Hello ");
    } else {
        if let Some(42) = Some(42) {
            println!("world")
        }
        else {
            println!("!")
        }
    }
    //~^^^^^^^^ collapsible_else_if

    if x == "hello" {
        print!("Hello ");
    } else {
        #[cfg(not(roflol))]
        if y == "world" {
            println!("world!")
        }
    }
}

#[rustfmt::skip]
fn issue_7318() {
    if true { println!("I've been resolved!")
    }else{
        if false {}
    }
    //~^^^ collapsible_else_if
}

fn issue14799() {
    use std::ops::ControlFlow;

    let c: ControlFlow<_, ()> = ControlFlow::Break(Some(42));
    if let ControlFlow::Break(Some(_)) = c {
        todo!();
    } else {
        #[cfg(target_os = "freebsd")]
        todo!();

        if let ControlFlow::Break(None) = c {
            todo!();
        } else {
            todo!();
        }
    }
}
