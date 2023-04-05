impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
         if strs.len() == 0{
             return String::from("");
             
        }
      
      
        let mut num =strs[0].len() ;
        if strs[0].len()  ==0{
                  return String::from("");
        }
        for x in 1..strs.len() {
            if strs[x].len()  ==0{
                  return String::from("");
            } 
             for (i, c) in  strs[x].chars().enumerate(){
                 if num==i{
                     break;
                 }
                 
                if  num <=strs[x].len() {
              
                 }
                 else {
              num =strs[x].len();}
              if  strs[0].chars().nth(i)==strs[x].chars().nth(i){
                  
                 }
                 else {
                       num=i;
                     
                     
                 break;
                 }  
            } 
      } 
        if num==0{
             return String::from("");
            
        } return String::from(&strs[0][..num]
        );
    } 
}
