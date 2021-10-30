use std::convert::TryInto;

pub fn Binary_Search(list :  &[i32],l : usize,h : usize,x :i32,) ->i32{
    let low  = l;
    let high = h;
    if high >= low {
        let  mid =  (low + high)/2;
    
         if list[mid] == x{

            return mid.try_into().unwrap();
         
        }else {
    
                if list[mid] > x{
                     return Binary_Search(list,l,mid+1,x);
                }else{
                    return Binary_Search(list,mid-1 , h, x);
                }
            }
        
    }else{
         return -1;
    }

}

    
