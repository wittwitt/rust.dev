fn main() {
    let i = 10_10;
    assert_eq!(i + 1, 1011);

    let j = 1_1_1_1;
    assert_eq!(j, 1111);

    let k = 0x1_1;
    assert_eq!(k, 17);
}
