#[derive(Debug)]
 struct Borrowed<'a> {
     x: &'a i32,
 }

// why???
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

fn main() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}

