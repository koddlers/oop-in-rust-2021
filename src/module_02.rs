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