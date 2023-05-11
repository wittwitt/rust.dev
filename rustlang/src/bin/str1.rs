fn main() {
    let s1 = r#"

 原始，不转意，多行字符串   
    
"#;
    println!("{}", s1);

    let s2 = r"af";
    println!("{}", s2);

    let s3 = r###"hah "#" # "##
    hah"###;
    println!("{}", s3);
}

#[test]
fn t1() {
    let s1 = "abc".to_string();
    let s2 = &s1;

    let s3 = "abc".to_string();
    let s4 = "abcd".to_string();

    assert_eq!(true, s2 == &s3);
    assert_eq!(false, s2 == &s4);

    assert_eq!(true, *s2 == s3);
    assert_eq!(false, *s2 == s4);
}
