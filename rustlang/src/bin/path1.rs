use std::path::{Path, PathBuf};

fn main() {
    let path = Path::new("foo.tar.gz");

    assert_eq!(path.with_extension(""), PathBuf::from("foo.tar"));

    assert_eq!(path.with_extension("xz"), PathBuf::from("foo.tar.xz"));

    assert_eq!(
        path.with_extension("").with_extension("txt"),
        PathBuf::from("foo.txt")
    );

    let f1 = "a.b.c".to_string();
    let p1 = Path::new(f1.as_str());
    assert_eq!(p1.with_extension(".tmp"), PathBuf::from("a.b.c.tmp"));
}
