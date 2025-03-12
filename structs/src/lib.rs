#[cfg(test)]
mod tests {
    struct User {
        username: String,
        age: i32,
    }

    #[test]
    fn test_struct() {
        let user = User {
            username: String::from("lomtom"),
            age: 18,
        };
        println!("username: {}, age: {}", user.username, user.age);
    }

    #[test]
    fn test_mutable() {
        let mut user = User {
            username: String::from("lomtom"),
            age: 18,
        };
        println!("username: {}, age: {}", user.username, user.age);
        user.age = 19;
        println!("username: {}, age: {}", user.username, user.age);
    }


    #[test]
    fn test_new_user() {
        let username = String::from("lomtom");
        let user = User {
            username,
            age: 18,
        };
        println!("username: {}, age: {}", user.username, user.age);
    }

    #[test]
    fn test_update_user() {
        let user = User {
            username: String::from("lomtom"),
            age: 18,
        };
        let update_user = User {
            age: 19,
            ..user
        };
        println!("username: {}, age: {}", update_user.username, update_user.age);
    }

    struct Dog(String, i32);

    #[test]
    fn test_method() {
        let user = Dog(String::from("xiaobai"), 2);
        println!("username: {}, age: {}", user.0, user.1);
    }

    #[test]
    fn test_unit_struct() {
        #[derive(Debug)]
        struct UnitStruct;
        let unit_struct = UnitStruct;
        println!("unit_struct {:?}", unit_struct);
    }

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    #[test]
    fn test_method_impl() {
        let rect = Rectangle {
            width: 10,
            height: 20,
        };
        println!("area: {}", rect.area());
    }

    impl Rectangle {
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle {
                width,
                height,
            }
        }
    }

    #[test]
    fn test_method_impl2() {
        let rect = Rectangle::new(10, 20);
        println!("area: {}", rect.area());
    }
}
