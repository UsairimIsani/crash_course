pub enum Cons<T> {
    Node { v: T, list: Box<Cons<T>> },
    Null,
}
impl<T> Hell for Cons<T> {}

impl<T: Hell> Cons<T> {}

pub trait Hell {}

struct NoCon;

impl Hell for NoCon {}

fn main() {
    // {
    //     let mut a: Cons<u8> = Cons::Null;
    //     a = ;

    //     let a: Cons<u32> = Cons::Null;
    // }

    let a: Vec<Box<dyn Hell>> = vec![
        Box::new(Cons::Node {
            v: 5,
            list: Box::new(Cons::Null),
        }),
        Box::new(NoCon),
    ];

    let a = Some(8);
    let a: Result<u8, Box<dyn std::error::Error>> = Ok(8);
}
