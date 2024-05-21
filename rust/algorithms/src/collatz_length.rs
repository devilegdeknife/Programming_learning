pub mod collatz_length {
    use std::io;

    pub fn read_number() -> i32 {
        let mut input_string = String::new();

        // 读取用户输入
        println!("请输入一个数字：");
        io::stdin().read_line(&mut input_string).expect("Failed to read line");

        // 将输入的字符串转换为数字，例如 i32 类型
        let number: i32 = input_string.trim().parse().expect("请输入有效的数字！");
        number
    }
    pub fn collatz_length(mut n : i32) -> u32 {
        let mut count:u32 = 1;

        while n != 1 {
            if n & 1 == 0 {
                n = n / 2;
            } else {
                n = n * 3 + 1;
            }
            count += 1;
        }

        count
    }
}
