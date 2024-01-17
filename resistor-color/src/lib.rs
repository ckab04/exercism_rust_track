use enum_iterator::IntoEnumIterator;
use std::collections::HashMap;


#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoEnumIterator)]
pub enum ResistorColor {
    
 Black,
 Brown,
 Red,
 Orange,
 Yellow,
 Green,
 Blue,
 Violet,
 Grey,
 White,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    let val = match _color {
        ResistorColor::Black=> 0,
        ResistorColor::Brown=> 1,
        ResistorColor::Red=>   2,
        ResistorColor::Orange=> 3,
        ResistorColor::Yellow=> 4,
        ResistorColor::Green=> 5,
        ResistorColor::Blue=>  6,
        ResistorColor::Violet=>  7,
        ResistorColor::Grey=>  8,
        ResistorColor::White=> 9
    };
    val 
}

pub fn value_to_color_string(value: usize) -> String {
    let hash = HashMap::from([
            (0, "Black"),
            (1, "Brown"),
            (2, "Red"),
            (3, "Orange"),
            (4, "Yellow"),
            (5, "Green"),
            (6, "Blue"),
            (7, "Violet"),
            (8, "Grey"),
            (9, "White")
    ]);
    let val = hash.get(&value).unwrap_or(&"value out of range");
    val.to_string()
}

pub fn colors() -> Vec<ResistorColor> {
   let vec = ResistorColor::into_enum_iter().collect();

   vec 
}
