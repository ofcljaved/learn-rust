fn main() {
    let s1 = String::from("Who's my owner");
    //At this point the owner of this string is s1 in stack

    let s2 = s1;
    //Now at this moment ownership is passed to s2
    //making s1 invalid to access
    //println!("{}", s1);
    //Try uncommenting ^ this line

    println!("s2 is new owner{}", s2);
    

    let b1 = String::from("Borrow me please!!");
    borrow_str(&b1);

    println!("Am i still owning you ? {}", b1);

    // So basically in case of passing ownership,
    // u pass value by call, like literally transfer ownership
    // but in case of borrowing it's call by reference
    // so that owner still in control of the data
    // and other person just borrow it use and leave
    


    let mut m1 = String::from("Hi");
    let m2 = &mut m1;
    //let m3 = m1;
    //So you cannot changer ownership after borrowing
    println!("{}", m2);

}

fn borrow_str(str: &String) {
    println!("Lemme borrow it {}", str);
}
