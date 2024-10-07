fn main() {
    println!("the length of string javed is {}", get_str_len("javed"));
}

fn get_str_len(str:&str) -> usize {
    return str.len();
}
