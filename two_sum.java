
class Solution {
    public int[] twoSum(int[] nums, int target) {
        
            int [] s = new int [2];
            
            for(int i=0;i < nums.length;i++)
            {
               for  (int g =0;g< nums.length;g++)
               {
                  if (g!=i)
                  {
                   if  ( nums[i]+nums[g]==target)
                   {
                      s[0]=i; s[1]= g;
                       return s;
                   }
                      
                  }
               }
         
            } return s;
}
    
}
