use std::sync::{Arc, Mutex, RwLock};
use std::thread::{spawn, JoinHandle};
fn main() {
    let a = Arc::new(Mutex::new(5));
    let handles = (1..=10)
        .map(|f| {
            let b = a.clone();
            spawn(move || {
                let mut c = b.lock().unwrap();
                *c += f;
                println!("Thread {} {}", f, c);
            })
        })
        .collect::<Vec<JoinHandle<()>>>();

    handles.into_iter().for_each(|f| {
        f.join().unwrap();
    });

    let a = Arc::new(RwLock::new(5));
    let handles = (1..=10)
        .map(|f| {
            let b = a.clone();
            spawn(move || {
                let c = b.read().unwrap();
                let d = b.read().unwrap();
                let e = b.read().unwrap();
                // *c += f;
                println!("Thread {} {} {} {}", f, c, d, e);
            })
        })
        .collect::<Vec<JoinHandle<()>>>();

    handles.into_iter().for_each(|f| {
        f.join().unwrap();
    });
}
