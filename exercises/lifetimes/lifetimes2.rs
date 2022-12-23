// lifetimes2.rs
//
// So if the compiler is just validating the references passed
// to the annotated parameters and the return type, what do
// we need to change?
//
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a hint.


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    //let string2 = String::from("xyz"); //This line can be simply extracted from the block 

    //{ //the block can be removed in this case, and there is no need to move its contents
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        //println!("The longest string is '{}'", result); // or the print statement can be put inside the block

    //} //all three options are viable in this case, but it may vary on a case by case basis
    println!("The longest string is '{}'", result);
}
