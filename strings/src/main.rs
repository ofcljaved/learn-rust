fn main() {
    let sting = String::from("hello world@");
    let word = first_word(&sting);
    println!("First word is {}",word );
}

fn first_word(str:&String) -> &str {
    for (i, char) in str.chars().enumerate() {
        if char == ' ' {
            return &str[0..i];
        } 
    }
    str
}
