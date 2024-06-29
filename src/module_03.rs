pub mod implementing_polymorphism_using_traits {
    trait Draw {
        fn draw(&self);
    }

    struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }

    impl Screen {
        pub fn render(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    struct Button {
        width: u32,
        height: u32,
        label: String,
    }

    impl Draw for Button {
        fn draw(&self) {
            println!("** Button: {} **\n", self.label);
        }
    }

    struct ListBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }

    impl Draw for ListBox {
        fn draw(&self) {
            println!("** ListBox **");
            println!("   ------- ");

            for option in self.options.iter() {
                println!("    {}", option);
            }
            println!();
        }
    }

    pub fn demo_polymorphism_using_trait_objects_in_action() {
        let screen = Screen {
            components: vec![
                Box::new(ListBox {
                    width: 100,
                    height: 20,
                    options: vec![
                        String::from("Apple"),
                        String::from("Orange"),
                        String::from("Banana"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 30,
                    label: String::from("OK"),
                }),
                Box::new(Button {
                    width: 50,
                    height: 30,
                    label: String::from("Cancel"),
                }),
            ]
        };
        screen.render();
    }
}