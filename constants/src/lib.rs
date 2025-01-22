#[cfg(test)]
mod tests {

    #[test]
    fn constant() {
        const C: i32 = 100;
        println!("the const is {}", C);
    }

    #[test]
    fn shadow() {
        const C: i32 = 100;
        println!("the const is {}", C);
        const C: i32 = 123;
        println!("the const is {}", C);
    }
}
