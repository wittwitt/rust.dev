use std::borrow::Cow;

struct Cat<'a> {
    name: Cow<'static, str>,
    sex: &'a str,
}

impl<'a> Cat<'a> {
    fn myname(&self) -> &str {
        return &self.name;
    }

    fn mysex(&'a self) -> &'a str {
        return &self.sex;
    }
}

fn main() {
    let cat = Cat {
        name: Cow::Borrowed("abc"),
        sex: "boy",
    };

    let cat_name = cat.name.clone();

    println!("ok: {:?}", cat.name);

    println!("{}, {},{}", cat_name, cat.myname(), cat.mysex());

    change_gas(&cat);
}

fn change_gas(cat: &Cat) {
    let nn = &cat.name;
    change_gas2(cat);
    println!("gas1: {}", nn);
}

fn change_gas2(cat: &Cat) {
    println!("gas2: {}", cat.name);
}
