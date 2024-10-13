use std::cell::RefCell;

fn main() {
    let ref_cell = RefCell::new("hello".to_string());

    let r = ref_cell.borrow();
    let count = r.len();
    assert_eq!(count, 5);
    // drop(r);

    let mut w = ref_cell.borrow_mut();
    w.push_str(" world");
}
