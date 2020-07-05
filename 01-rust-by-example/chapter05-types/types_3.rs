fn main() {
    let elem = 5u8;

    let mut vec = Vec::new();
    vec.push(elem);     // without this line, error will occur at line 6
    println!("{:?}", vec);
}
