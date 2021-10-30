// use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    value: String,
    next: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

#[derive(Clone)]
pub struct Linkedlist {
    pub head: Link,
}
// impl std::fmt::Display for Linkedlist{
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
impl Linkedlist {

    pub fn display(&mut self){
        self.head.take().map(|head| {
            
                println!("{:?} -->",head);
            
            // if let Some(next) = head.borrow_mut().next.take() {

            //     self.head = Some(next);

            // } else {
            //     break;
                // self.head = None;
        });
        // let current  = self.head;
        // while let Some(x) = current {

        //     println!("{:?} -->",x);
        //     current =  current.borrow().next;


        // 
    }



    
    pub fn inertFirst(&mut self, value: String) {

        let new = Node::new(value);
        
        match self.head.take() {

            Some(old) => {
            
                new.borrow_mut().next = Some(old.clone());  
            
                self.head = Some(new);
            
            }, 
            
            None => self.head = Some(new.clone())
        
        };    
    
    }

    pub fn deleteFirst(&mut self) -> Option<String> {
    
        self.head.take().map(|head| {
    
            if let Some(next) = head.borrow_mut().next.take() {
    
                self.head = Some(next);
    
            } else {
    
                self.head = None;
    
            }
    
            Rc::try_unwrap(head)
    
            .ok()
    
            .expect("I think List is Empty!")
    
            .into_inner()
    
            .value
    
        })
    
        
    
    }

    
    pub fn new_empty() -> Linkedlist {
    
        Linkedlist { head: None}
    
    } 

}
