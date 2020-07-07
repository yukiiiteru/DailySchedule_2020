fn main() {
    println!("Mary had a little lamb.");
    println!("Its fleece was white as {}", "snow");
    println!("And everywhere that Mary went.");
    println!("{0}{0}{0}{0}{0}{0}{0}{0}{0}{0}", ".");

    let end1 = String::from("C");
    let end2 = "h";
    let end3 = "e";
    let end4 = "e";
    let end5 = "s";
    let end6 = "e";
    let end7 = String::from("B");
    let end8 = "u";
    let end9 = "r";
    let end10 = "g";
    let end11 = "e";
    let end12 = "r";

    println!("{}", end1 + end2 + end3 + end4 + end5 + end6);
    println!("{}", end7 + end8 + end9 + end10 + end11 + end12);
}
