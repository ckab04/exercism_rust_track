use time::PrimitiveDateTime as DateTime;
use time::Duration;


// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    //unimplemented!("What time is a gigasecond later than {}", start);

    
   start.checked_add(Duration::seconds(1000000000)).unwrap()
  
}
