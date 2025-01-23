#[cfg(test)]
mod tests {

    fn type_of<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
    }

    #[test]
    fn ints() {
        // 基础类型
        let i8: i8 = -1;
        let i16: i16 = -1;
        let i32: i32 = -1;
        let i64: i64 = -1;
        let i128: i128 = -1;
        let isize: isize = -1;
        println!(
            "有符号 i8: {}, i16: {}, i32: {}, i64: {}, i128: {}, isize: {}",
            i8, i16, i32, i64, i128, isize
        );
        let u8: u8 = 1;
        let u16: u16 = 1;
        let u32: u32 = 1;
        let u64: u64 = 1;
        let u128: u128 = 1;
        let usize: usize = 1;
        println!(
            "无符号 u8: {}, u16: {}, u32: {}, u64: {}, u128: {}, usize: {}",
            u8, u16, u32, u64, u128, usize
        );
        // 默认类型
        let default_integer = 42; // 推断为 i32
        println!("default_integer type: {}", type_of(default_integer));
        // 下划线
        let large_number = 1_000_000; // 更易读
        println!("large_number type: {}", type_of(large_number));
        // 类型转换
        let x: u8 = 255;
        let y: u16 = x as u16; // 强制转换
        println!("x type: {}, y type: {}", type_of(x), type_of(y));
    }

    // 溢出
    #[test]
    fn overflow() {
        let x: u8 = 255;
        let y = x + 1; // 调试模式会崩溃，发布模式结果为 0
        println!("x: {}, y: {}", x, y);
    }

    // 溢出处理
    #[test]
    fn overflow_handling() {
        let signed: i32 = -100;
        let unsigned: u32 = 100;

        println!("Signed: {}, Unsigned: {}", signed, unsigned);

        // 溢出示例
        let max: u8 = 255;
        let wrap_around = max.wrapping_add(1);
        println!("Wrap around: {}", wrap_around); // 输出 0
    }

    #[test]
    fn float() {
        // 浮点类型
        let single_precision: f32 = 3.14;
        let double_precision: f64 = 3.14;
        println!(
            "single_precision type: {}, double_precision type: {}",
            type_of(single_precision),
            type_of(double_precision)
        );
        // IEEE 754 标准
        let infinity = 1.0 / 0.0; // 正无穷大
        let negative_infinity = -1.0 / 0.0; // 负无穷大
        let not_a_number = 0.0 / 0.0; // NaN
        println!(
            "infinity: {}, negative_infinity: {}, not_a_number: {}",
            infinity, negative_infinity, not_a_number
        );
        // 科学计数法
        let scientific = 1.2e3; // 等价于 1.2 × 10^3，即 1200
        let small_number = 4.5e-3; // 等价于 4.5 × 10^-3，即 0.0045
        println!("scientific: {}, small_number: {}", scientific, small_number);
        // 运算
        let x = 2.5;
        let y = 1.5;
        println!("加法: {}", x + y);
        println!("减法: {}", x - y);
        println!("乘法: {}", x * y);
        println!("除法: {}", x / y);
        // 四舍五入
        let x = std::f32::consts::PI;
        println!("向下取整: {}", x.floor());
        println!("向上取整: {}", x.ceil());
        println!("四舍五入: {}", x.round());
        // 绝对值
        let x: f32 = -5.67;
        println!("绝对值: {}", x.abs());
        // 幂运算
        let x: f32 = 2.0;
        println!("平方: {}", x.powi(2)); // 输出 4.0
        println!("开平方: {}", x.sqrt()); // 输出 1.414213...
    }

    #[test]
    fn boolean() {
        let is_rust_fun: bool = true;
        let is_boring: bool = false;
        println!("Rust is fun: {}", is_rust_fun);
        println!("Is Rust boring? {}", is_boring);
    }

    // 字符类型
    #[test]
    fn char() {let c1: char = 'a';        // 单个 ASCII 字符
        let c2: char = '中';       // 中文字符
        let c3: char = '🚀';       // Emoji 字符
        println!("c1: {}, c2: {}, c3: {}", c1, c2, c3);
    }

    // 复合类型（元组）
    #[test]
    fn tuple() {
        let tuple: (i32, f64, char) = (42, 3.14, 'a');
        println!("元组中的值：({}, {}, {})", tuple.0, tuple.1, tuple.2);
        let tuple = (42, 3.14, 'a');
        println!("第一个元素：{}", tuple.0); // 输出 42
        println!("第二个元素：{}", tuple.1); // 输出 3.14
        println!("第三个元素：{}", tuple.2); // 输出 'a'
        let tuple = (42, 3.14, 'a');
        let (x, y, z) = tuple;
        println!("解构后的值：{}, {}, {}", x, y, z); // 输出 42, 3.14, 'a'
    }

    // 复合类型（数组）
    #[test]
    fn array() {
        let arr: [i32; 3] = [1, 2, 3];
        println!("数组中的值：{}, {}, {}", arr[0], arr[1], arr[2]);
        let arr = [3; 5]; // 创建一个包含 5 个 3 的数组
        println!("{:?}", arr); // 输出 [3, 3, 3, 3, 3]
    }
}
