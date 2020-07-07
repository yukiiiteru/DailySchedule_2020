fn main() {
    let tabby_cat = "\tI'm tabbed in.";
    let persian_cat = "I'm split\non a line.";
    let backslash_cat = "I'm \\ a \\ cat.";

    let fat_cat = "I'll do a list:\n\t* Cat food\n\t* FIshes\n\t* Catnip\n\t* Grass";

    println!("{}", tabby_cat);
    println!("{}", persian_cat);
    println!("{}", backslash_cat);
    println!("{}", fat_cat);
}
