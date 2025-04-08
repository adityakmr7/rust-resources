fn main() {
    let string1 = String::from("Hello");
    let string2 = String::from("World");
    let result = concatenate_strings(&string1,&string2);
    println!("Result {}", result);
}


fn concatenate_strings(s1: &String, s2: &String) -> String  {
    let mut result:String = String::from("");
    result.push_str(s1);
    result.push_str(s2);   
    return result;
}