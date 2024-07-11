fn main() {
    let _x: i8 = 127;
    let age: u8 = 22;
    let name: String = String::from("Javed");
    let is_male: bool = true;
    println!("My name is {}, and the age is {}", name, age);

    if is_male {
        println!("You're a male");
    } else {
        println!("You're not a male");
    }

    println!("Here is the table of 2");
    for i in 0..12 {
        if i == 0 {
            continue;
        } else if i > 10 {
            break;
        }
        println!("2 x {} = {}", i, 2 * i);
    }
}
