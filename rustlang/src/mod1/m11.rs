pub struct Cat {}

impl Cat {
    // 重复 ，编译不通过
    // pub fn say(&self) {
    //     println!("cat m11 say")
    // }

    pub fn say2(&self) {
        println!("cat m11 say2")
    }
}
