// pub mod Binary_search;
mod LinkedList;
// use Binary_search::Binary_Search;
use LinkedList::*;
fn main() {
    let mut x = Linkedlist::new_empty();
    
    x.inertFirst(7.to_string());
    x.inertFirst("huz".to_string());
    x.inertFirst(2.to_string());
    x.inertFirst(3.to_string());
    x.inertFirst(4.to_string());
    x.inertFirst(5.to_string());

    // x.deleteFirst();
    x.display();
}
