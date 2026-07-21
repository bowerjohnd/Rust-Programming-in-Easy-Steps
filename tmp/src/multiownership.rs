// Rc, single threaded
// Arc, mutli threaded
// RefCell borrows dynamically

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let ans = Rc::new(RefCell::new(6*9));
    let c1 = ans.clone();

    c1.replace(42);

    println!("ans = {} c1 = {}", *ans.borrow(), *c1.borrow());
}

// examples of locking multi-threading borrows

// let quit = Arc::new(Mutex::new(false));
// cloned_quit = quit.clone()

// let mut data = cloned_quit.lock().unwrap();
// *data = true;
