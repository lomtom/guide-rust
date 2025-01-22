
#[cfg(test)]
mod tests {

    #[test]
    fn immutable() {
        let x = 5;
        println!("The value of x is: {x}");
        // build fail
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
}
