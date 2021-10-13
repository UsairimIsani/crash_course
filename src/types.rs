// pub mod parkor {
//     #[derive(Debug)]
//     pub struct Gene; // Unit Struct

//     impl Default for Gene {
//         fn default() -> Self {
//             Self
//         }
//     }
//     #[derive(Debug, Default)]
//     pub struct Tuple(pub String, pub Gene);

//     #[cfg(test)]
//     mod tests {
//         #[test]
//         fn test_something() {
//             assert!(true)
//         }
//     }

//     impl From<(String, u64)> for Tuple {
//         fn from(t: (String, u64)) -> Self {
//             Self(t.0, t.1)
//         }
//     }
//     impl From<[u64; 2]> for Tuple {
//         fn from(t: [u64; 2]) -> Self {
//             Self(t[0].to_string(), t[1])
//         }
//     }

//     impl Tuple {
//         pub fn new() -> Self {
//             Self(Default::default(), Default::default())
//         }
//     }

//     #[test]
//     pub fn chicken() {
//         println!("klaskldf");
//     }

//     pub trait TestTrait<T> {
//         type Apple;

//         fn test_trait(p: T) -> Self::Apple;
//     }

//     impl TestTrait<u64> for Gene {
//         type Apple = String;

//         fn test_trait(p: u64) -> Self::Apple {
//             p.to_string()
//         }
//     }
//     pub struct St {
//         pub name: Tuple,
//     }
// }
