extern crate proc_macro;

use proc_macro::*;

///
/// 1. 必须位于the root of crate
/// 2. [lib] proc-macro = true
/// 3. 不能在定义的crate里使用，只能外部使用
///

#[proc_macro_attribute]
pub fn say_hi(args: TokenStream, input: TokenStream) -> TokenStream {
    let x = format!(
        r#"
        fn dummy() {{
            println!("====entering");
            println!("====args tokens: {{}}", {args});
            println!("====input tokens: {{}}", {input});
            println!("====exiting");
        }}

        fn f2(){{
            println!("====f2,hahah");
        }}
    "#,
        args = args.into_iter().count(),
        input = input.into_iter().count(),
    );

    x.parse().expect("Generated invalid tokens")
}

mod t {
    // use super::say_hi;

    // #[say_hi]
    // fn say() {
    //     println!("this si say method")
    // }

    #[test]
    fn t1() {
        // say();
    }
}
