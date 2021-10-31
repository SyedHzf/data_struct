use std::{cell::RefCell, rc::Rc};


struct DoublyLinkedList{
    first:Link,
    last:Link
}
impl DoublyLinkedList{
    fn insert_first(&mut self,value:i32){
       let node = Node::new(value);
        match self.first.take(){
            Some(old) => {
                node.borrow_mut().next = Some(old.clone());
                old.borrow_mut().prev = Some(node.clone());

            }
            None => {
                self.last = Some(node.clone());
        }

    };
    self.first = Some(node);

}

    fn insert_last(&mut self,value: i32){
        let node = Node::new(value);
        match self.last.take(){
           
            Some(old) => {
               
                node.borrow_mut().prev = Some(old.clone());
               
                old.borrow_mut().next = Some(node.clone());

            }

            None => {
                self.first = Some(node.clone());
        }

    };
    self.last = Some(node);
 
    }

    fn delete_first(&mut self){
        self.first.take().map(|head|{
            if let Some(new) = head.borrow_mut().next.take(){
                new.borrow_mut().prev = None;
                self.first = Some(new);

            }else {
                self.last = None;
            }
        }
    );

}
    fn delete_last(&mut self){
        self.last.take().map(|tail|{
            
            if let Some(new) = tail.borrow_mut().prev.take(){
                
                new.borrow_mut().next = None;
                self.last = Some(new);
            
            }else {
            
                self.first = None;
            
                self.last = None;
            
            }
        }
    );
    }
    }
pub struct Node {
    value: i32,
    next: Link,
    prev:Link
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
            prev: None
        }))
    }
}
