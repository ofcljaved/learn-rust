fn main() {
    let s1 = String::from("Who's my owner");
    //At this point the owner of this string is s1 in stack
    
    let s2 = s1;
    //Now at this moment ownership is passed to s2
    //making s1 invalid to access
    //println!("{}", s1);
    //Try uncommenting ^ this line
    

    println!("s2 is new owner{}", s2);
}
