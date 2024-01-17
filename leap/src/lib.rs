pub fn is_leap_year(year: u64) -> bool {
    let val = year % 4;

   if val == 0{
    if  year % 100 == 0 && year % 400 != 0 {
        return false;
    }

    return true;
   }
  else
    {
       return false;
    }
      
        
}
