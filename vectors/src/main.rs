fn main() {
    let mut num = vec!["hello".to_string(), "world".to_string()];
    even_vec(&mut num);
    println!("{:?}",num);
}

fn even_vec(v:&mut Vec<String>) {
    let mut even: Vec<&String> = Vec::new();
    for i in v {
        i.push_str("javed");
        even.push(i);
    }
    println!("{:?}", even)
}
