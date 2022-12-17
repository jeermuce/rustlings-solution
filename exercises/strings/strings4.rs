// strings4.rs

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");//returns slice

    string("red".to_string());//returns String

    string(String::from("hi"));//returns String

    string("rust is fun!".to_owned());//returns String

    string_slice("nice weather".into());//returns both because into() tries to convert what it gets into what we want
    string("nice weather".into());//returns both because into() tries to convert what it gets into what we want

    string(format!("Interpolation {}", "Station"));//returns String

    string_slice(&String::from("abc")[0..1]);//returns slice

    string_slice("  hello there ".trim());//returns slice because trim doe not mutate the string, it changes our view of it

    string("Happy Monday!".to_string().replace("Mon", "Tues"));//returns String

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());//returns String



}
