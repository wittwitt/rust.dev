fn main() {
    // f1();
    f12();
}

#[allow(dead_code)]
fn f13() {
    let mut arr1: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut sl1 = &mut arr1[2..5];
    sl1[0] = 100;
    println!("sl1: {:?}", sl1);
}

#[allow(dead_code)]
fn f1() {
    let mut arr1: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // slice
    let sl1: &mut [i32] = &mut arr1[2..5];
    println!("sl1: {:?}", sl1);
    sl1[0] = 200;
    println!("sl1: {:?}", sl1);

    println!("arr1: {:?}", arr1);

    for item in arr1.iter() {
        println!("{}", item);
    }
}

#[allow(dead_code)]
fn f12() {
    let arr1: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // slice
    let sl1: &[i32] = &arr1[2..5];
    println!("sl1: {:?}", sl1);
    println!("arr1: {:?}", arr1);

    for item in arr1.iter() {
        println!("{}", item);
    }
}

#[allow(dead_code)]
fn f2() {
    {
        let s: &mut [i32] = &mut [1, 2, 3, 4];
        println!("{:?}", s);

        s[1] = 1000;

        println!("{:?}", s);
    }

    {
        let mut s = [1, 2, 3, 4, 5];
        let ss: &mut [i32] = &mut s[2..4];
        ss[0] = 10000;
        // s[2] = 3;
        println!("{:?}", ss)
    }

    {
        let mut s = [1, 2, 3, 4, 5];
        let ss: &[i32] = &s[2..4];
        // ss[0] = 10000;
        println!("{:?}", ss)
    }
}
