// 用两种方法求解
fn main() {
    never_return();
}

fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    loop{
        println!("你已经到了穷途末路，崩溃吧！")
    }
}