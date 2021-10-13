// use std::sync::{Arc, RwLock, Weak};

// /// # Node is Thread Safe Tree
// /// Hello World of chicken kebab
fn main() {}
// #[derive(Debug)]
// pub struct Node<T> {
//     value: T,
//     parent: RwLock<Weak<Node<T>>>,
//     children: RwLock<Vec<Arc<Node<T>>>>,
// }
// impl<T> Node<T> {
//     /// ## Creates a new Node
//     ///
//     /// ```
//     /// let node_1 = Node::new(3);
//     /// ```
//     pub fn new(value: T) -> Arc<Self> {
//         Arc::new(Node {
//             value,
//             parent: RwLock::new(Weak::new()),
//             children: RwLock::new(Vec::new()),
//         })
//     }

//     pub fn insert(parent: &mut Arc<Node<T>>, node: &mut Arc<Node<T>>) {
//         match parent.children.write() {
//             Ok(mut e) => {
//                 *node.parent.write().unwrap() = Arc::downgrade(parent);
//                 e.push(Arc::clone(node));
//             }
//             Err(e) => {
//                 println!("Errror {}", e);
//             }
//         }
//     }

//     pub fn get_value(&self) -> &T {
//         &self.value
//     }
// }

// #[cfg(test)]
// mod test {
//     #[test]
//     fn create_node() {
//         use super::*;
//         let _node_1 = Node::new(3);
//         let _node_2 = Node::new(String::new());
//         assert!(true);
//     }

//     #[test]
//     fn insert_node() {
//         use super::*;
//         let mut parent = Node::new(3);
//         let mut child = Node::new(56);
//         let mut grandchild = Node::new(78);
//         let mut child_2 = Node::new(796);
//         Node::insert(&mut child, &mut grandchild);
//         Node::insert(&mut parent, &mut child);
//         Node::insert(&mut parent, &mut child_2);

//         println!(
//             "Parent : {:?}\n\nChild : {:?}\n\nChild's Parent : {:?}",
//             parent,
//             child,
//             child.parent.read().unwrap().upgrade().unwrap()
//         );
//         let parent_val = parent.get_value();
//         let child_value = child.parent.read().unwrap();

//         assert_eq!(parent_val, child_value.upgrade().unwrap().get_value());
//     }
// }
