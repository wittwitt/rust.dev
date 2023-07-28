mod setting;

use std::env;

use setting::Setting;

fn main() {
    env::set_var("HAHA_ENV_name", "haha");
    env::set_var("HAHA_ENV_age", "100");
    let setting = Setting::new().unwrap();
    println!("Hello, world! {:?}", setting.name);
}
