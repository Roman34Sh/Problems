class Solution {
    public int romanToInt(String s) {
        char p;
        int q=0;
        int answer=0;
        for(int i=0;i<s.length();i++)
        { p = s.charAt(i);
       
       
      
            switch(p)
            {
 case 'I':
  q = 1;    
                    
    break;

 case 'V': if(q==1){
                 q = 3;
                }
        else
                 q=5;          
    break; 
 case 'X': if(q==1){
    q = 8;
          }
                    else
                        q=10;
    break;
 case 'L': if(q==10) 
 {
     q=30;
 } else
    q = 50;
   
    break;
 case 'C': if(q==10){
     q=80;
 } else
    q = 100;
   
    break;
 case 'D': if(q==100){
     q=300;
 } else
     q = 500;
   
    break;
 case 'M': if(q==100){
     q=800;
 } else
    q = 1000;
                    
       
    break;
  default:
   
            }
          
           answer = answer+q;
        
        
        }
          
        
            
        return answer;
        
        
            
        
        
    }
}
