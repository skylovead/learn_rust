//1、Rust中每一个引用都有其生命周期，也就是引用保持有效的作用域。大部分时候生命周期是隐含并可以推断的，正如大部分时候类型可以推断一样。
//
//2、生命周期的主要目标是避免悬垂引用。
//
//3、Rust编译器使用借用检查器来检查生命周期是否有效。
fn main() {
    ////错误例子1
    //let r;                       //---------------------------+------------'a
    //{                            //                           +
    //    let x = 5;               //-------+------'b           |
    //    r = &x;                  //       |                   |
    //}                            //-------+                   |
    //println!("r = {}", r);       //                           |
    //                             //                           |
    //println!("Hello, world!");   //                           |
    //                             //---------------------------+
    //
    let r;                       //---------------------------+------------'a
                                 //                           +
    let x = 5;                   //-------+------'b           |
    r = &x;                      //       |                   |
                                 //       |                   |
    println!("r = {}", r);       //       |                   |
                                 //       |                   |
    println!("Hello, world!");   //       |                   |
}                                //---------------------------+
