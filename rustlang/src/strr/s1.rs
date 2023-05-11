#[allow(dead_code)]
fn extract_boundary(content_type: &str) -> Option<&str> {
    const BOUNDARY: &str = "boundary=";
    content_type.find(BOUNDARY).map(|idx| {
        let start = idx + BOUNDARY.len();
        let end = content_type[start..]
            .find(';')
            .map_or(content_type.len(), |end| start + end);
        &content_type[start..end]
    })
}

#[test]
fn t() {
    // let mut c = "extract_boundary";
    let mut ct = "Content-Type: multipart/byteranges; boundary=3d6b6a416f9b5";

    let c2 = extract_boundary(&mut ct);

    match c2 {
        Some(cs) => {
            println!("cs{}", cs);
        }
        None => {
            println!("nonce");
        }
    }
}

#[test]
fn t2() {
    let str1 = "abcde";
    let fpos = str1.find("b");
    println!("{}", fpos.unwrap());

    let fpos = str1.find("x");
    println!("{}", fpos.unwrap_or(1000));

    let str1_len = str1.len();
    str1.find("b").map_or(str1_len, |find_pos| find_pos);
}
