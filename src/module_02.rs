pub mod creating_objects_in_rust {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn scale(&mut self, scale_factor: u32) {
            self.width *= scale_factor;
            self.height *= scale_factor;
        }
    }

    pub fn creating_custom_objects_and_invoking_methods() {
        let mut rect = Rectangle {
            width: 20,
            height: 10,
        };
        println!("Area of the rectangle is (before): {}", rect.area());

        rect.scale(2);
        println!("Area of the rectangle is (after): {}", rect.area());
    }
}

pub mod geometry {
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            Self {
                width,
                height
            }
        }

        pub fn area(&self) -> u32 {
            self.width * self.height
        }
    }
}

pub mod creating_objects_in_rust_v2 {
    use crate::module_02::geometry;

    pub fn demo_visibility_of_objects_and_their_parts_inside_modules() {
        let mut rect = geometry::Rectangle {
            width: 50,
            height: 60,
        };
        println!("Area of the rectangle is: {}", rect.area());

        let mut rect_2 = geometry::Rectangle::new(200, 400);
        println!("Area of the rect_2 is: {}", rect_2.area());
    }
}