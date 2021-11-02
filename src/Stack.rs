type counter = i32;
struct Stackx<T>
{

    stack : Vec<T>
    

}

impl<T> Stackx<T>{

    fn new() -> Self{

       Stackx{

       stack : Vec::new()
    }
        }
    
    
    fn push(&mut self,val:T) {

        self.stack.push(val);

    }

    fn pop(&mut self ) -> Option<T>{

        self.stack.pop()

    }

    fn length(&mut self) -> usize{
        self.stack.len()
    }

    fn peek(&mut self) -> Option<&T>{

        self.stack.last()
    
    }

}
#[cfg(test)]
mod test{
    #[test]
    fn checking_pushing_poping(){
        let mut s1 = super::Stackx::<i32>::new();
        s1.push(3);
        assert_eq!(3,s1.pop().unwrap());

    }
}