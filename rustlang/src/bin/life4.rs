fn main() {}

// 1. 生命周期是一种特殊的泛型
// 2. 泛型是early bound
// 函数声明/传递/调用，，，泛型在传递的时候，已经确定T了，，所以是early bound
// 生命周期
//      默认 late bound
// early bound：
//      1. 生命周期参数受到它必须超过的某个其他生命周期的限制。
//      2. 生命周期在《函数签名之外声明》，例如 在结构的关联方法中，它可能来自结构本身

pub fn f<'a>() {} // late bound
pub fn g<'a: 'a>() {} // early bound

#[test]
fn t1() {
    // let pf = f::<'static> as fn();
    let pf = f as fn();

    // let pg = g::<'static> as fn();
    let pg = g as fn();
    println!("{}", pf == pg);
}

//-----------------------------------------------------------------------

pub fn the_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

pub fn the_longest_cp<'a, 'b: 'a>(s1: &'b str, s2: &'b str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

#[test]
fn t2() {
    let pf = the_longest as fn(&'static str, &'static str) -> &'static str;
    let pg = the_longest_cp::<'static, 'static> as fn(&'static str, &'static str) -> &'static str;
    println!("{}", pf == pg);
}

//-----------------------------------------------------------------------

pub struct Buffer<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'b, 'a: 'b> Buffer<'a> {
    fn new(b: &'a [u8]) -> Buffer {
        Buffer { buf: b, pos: 0 }
    }

    pub fn read_bytes_early(&'b mut self) -> &'a [u8] {
        self.pos += 3;
        &self.buf[self.pos - 3..self.pos]
    }

    pub fn read_bytes_late<'c>(&'c mut self) -> &'c [u8] {
        self.pos += 3;
        &self.buf[self.pos - 3..self.pos]
    }
}

#[test]
fn t3() {}
