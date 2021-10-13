pub mod lock;
pub mod parser;
pub mod types;

pub struct Animal {
    pub name: String,
    pub ty: u8,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
