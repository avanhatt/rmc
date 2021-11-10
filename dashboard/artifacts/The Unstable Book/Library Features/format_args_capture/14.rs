// compile-flags: --edition 2018
#![allow(unused)]
#![feature(format_args_capture)]

pub fn main() {
let (person, species, name) = ("Charlie Brown", "dog", "Snoopy");

// captures named argument `person`
print!("Hello {person}");

// captures named arguments `species` and `name`
format!("The {species}'s name is {name}.");
}