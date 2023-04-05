impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut s: i32= 0;
         let mut x1: i32= x;
         let mut a: i32= 0;
    while x1>0{
        
        s=x1%10;
        x1= x1/10;
        a= (a*10)+s; 
        } 
        if x==a {
            return true;
        }
        return false;
        
        
    }
}
