fn main() {
    let num = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter = num.iter().filter(|x| *x % 2 != 0).map(|x| x * 2);

    let mut updated_vec = Vec::new();
    for val in iter {
        updated_vec.push(val);
    }
    println!("Old vector{:?}", num);
    println!("New vector{:?}", updated_vec);
}
