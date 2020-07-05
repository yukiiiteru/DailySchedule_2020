fn main() {
    let mut _mutable_integer = 7i32;

    {
        let large_integer = &_mutable_integer;

        // _mutable_integer = 50;

        println!("Immutably borrowed {}", large_integer);
    }

    _mutable_integer = 3;
}

