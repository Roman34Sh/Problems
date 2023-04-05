impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut vec = Vec::new();
        for ch in s.chars(){
          match ch {
              '{' |'['|'(' =>   vec.push(ch),
              '}' => if  vec.pop() != Some('{') {return false;},
              ']' => if  vec.pop() != Some('[') {return false;},
              ')' => if  vec.pop() != Some('(') {return false;},
              _=> return false,
            }; 
            
        } return vec.is_empty();
        
            }
}
