struct A;

struct Single(A);

struct SingleGen<T>(T);

fn main() {
    let _s = Single(A);
    
    let _char: SingleGen<char> = SingleGen('a');

    let _t    = SingleGen(A);
    let _i32  = SingleGen(6);
    let _char = SingleGen('a');
}

