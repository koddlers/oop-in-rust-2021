#![allow(unused)]

mod module_02;
mod module_03;
mod module_04;

use module_02::creating_objects_in_rust;
use module_02::creating_objects_in_rust_v2;
use module_03::implementing_polymorphism_using_traits;
use module_04::a_practical_oop_example_implementing_the_observer_design_pattern as oop_observer;

fn main() {
    // Module 02 - Creating Objects in Rust
    // creating_objects_in_rust::creating_custom_objects_and_invoking_methods();
    // creating_objects_in_rust_v2::demo_visibility_of_objects_and_their_parts_inside_modules();

    // Module 03 - Implementing Polymorphism Using Traits
    // implementing_polymorphism_using_traits::demo_polymorphism_using_trait_objects_in_action();

    // Module 04 - A Practical OOP Example - Implementing the Observer Design Pattern
    oop_observer::demo_observer_design_pattern();
}
