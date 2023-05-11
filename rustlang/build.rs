fn main() {
    #[cfg(feature = "hupage")]
    println!("cargo:rustc-link-lib=hugetlbfs");
}
