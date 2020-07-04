fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;
    let print = || println!("{}", x);

    apply(print);
}

