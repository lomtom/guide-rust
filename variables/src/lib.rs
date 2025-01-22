#[cfg(test)]
mod tests {

    #[test]
    fn immutable() {
        let x = 5;
        println!("The value of x is: {x}");
        // check fail
        // x = 6;
        // println!("The value of x is: {x}");
    }

    #[test]
    fn mutable() {
        let mut x = 5;
        println!("The value of x is: {x}");
        x = 6;
        println!("The value of x is: {x}");
    }

    #[test]
    fn shadow() {
        let x = 5;
        println!("The value of x is: {x}");
        let x = x + 1;
        println!("The value of x is: {x}");
        let x = x * 2;
        println!("The value of x is: {x}");
    }

    #[test]
    fn shadow_scope() {
        let  x = 5;
        {
            let x = "six";
            println!("The value of x is: {x}");
        }
        println!("The value of x is: {x}");
    }

    #[test]
    fn uninitialized() {
        let x;
        // fail
        // println!("The value of y is: {x}");
        x = 6;
        println!("The value of y is: {x}");
    }
}
