pub mod a_practical_oop_example_implementing_the_observer_design_pattern {
    /////////////////////////////////////////////////////////////////
    // A simple Rust implementation of the Observer Design Pattern //
    /////////////////////////////////////////////////////////////////

    // Trait for a generic Observer
    trait Observer {
        fn update(&self);
    }

    // Trait for a generic Subject
    trait Subject<'a, T: Observer> {
        fn attach_observer(&mut self, observer: &'a T);
        fn detach_observer(&mut self, observer: &'a T);
        fn notify_observers(&self);
    }

    // Implement a concrete subject: the FileSubject
    struct FileSubject<'a, T: Observer> {
        observers: Vec<&'a T>,
    }

    impl<'a, T: Observer + PartialEq> FileSubject<'a, T> {
        // Create a new FileSubject
        fn new() -> FileSubject<'a, T> {
            FileSubject {
                // Create a new empty vector
                observers: Vec::new()
            }
        }
    }

    // Implement the Subject trait for FileSubject
    impl<'a, T: Observer + PartialEq> Subject<'a, T> for FileSubject<'a, T> {
        fn attach_observer(&mut self, observer: &'a T) {
            // Add this new observer to the observers vector, to keep track of it
            self.observers.push(observer);
        }

        fn detach_observer(&mut self, observer: &'a T) {
            // Remove this observer from the observers vector;
            // The removed observer will NOT be notified of state changes from this subject anymore.
            if let Some(index) = self.observers.iter().position(|x| *x == observer) {
                self.observers.remove(index);
            }
        }

        fn notify_observers(&self) {
            // For each observer that is monitoring us:
            for observer in self.observers.iter() {
                // Notify it
                observer.update();
            }
        }
    }

    // Implement a concrete observer: the ObserverProcess
    #[derive(PartialEq)]
    struct ObserverProcess {
        id: u32,
        name: String,
    }

    // Implement the Observer trait for the ObserverProcess
    impl Observer for ObserverProcess {
        fn update(&self) {
            println!("Observer {} (id: {}) received notification from file.", self.name, self.id);
        }
    }


    // Lesson "07 - Demo - The Observer Object-oriented Design Pattern in Rust in Action"
    pub fn demo_observer_design_pattern() {
        // Create the subject
        println!("Creating the subject.");
        let mut file_subject = FileSubject::new();

        // Create the observers
        println!("Creating the observers.");
        let antivirus_observer = ObserverProcess { id: 10, name: String::from("Antivirus") };
        let cloudfs_observer = ObserverProcess { id: 20, name: String::from("Cloud FS") };
        let editor_observer = ObserverProcess { id: 30, name: String::from("Editor") };

        // Attach the observers to the subject
        println!("Attaching AV, Cloud FS, and editor observers.");
        file_subject.attach_observer(&antivirus_observer);
        file_subject.attach_observer(&cloudfs_observer);
        file_subject.attach_observer(&editor_observer);
        println!();

        // Notify the observers of some change
        println!("Sending notifications to observers:");
        file_subject.notify_observers();
        println!();

        // Try detaching one observer
        println!("Detaching the Cloud FS.");
        file_subject.detach_observer(&cloudfs_observer);
        println!();

        // Try notifying again the remaining observers
        println!("Sending notifications to observers:");
        file_subject.notify_observers();
    }
}