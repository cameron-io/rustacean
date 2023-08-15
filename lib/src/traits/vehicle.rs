trait Vehicle {
    // Constructor
    // -> `Self` refers to the implementor type.
    fn new(manufacturer: &'static str, model: &'static str, year: i32) -> Self;
    
    // Method signatures
    fn manufacturer(&self) -> &'static str;
    fn model(&self) -> &'static str;
    fn year(&self) -> i32;
    fn owned(&self) -> bool;

    // Default Trait
    fn noise(&self) -> &'static str {
        "vroom"
    }
}

pub struct Car {
    model: &'static str,
    manufacturer: &'static str,
    year: i32,
    owned: bool
}

impl Car {
    pub fn purchase(&mut self) {
        self.owned = true;
    }
}

// Implement the `Vehicle` trait for `Car`.
impl Vehicle for Car {
    // `Self` is the implementor type: `Car`.
    fn new(manufacturer: &'static str, model: &'static str, year: i32) -> Car {
        Car {
            model: model,
            owned: false,
            manufacturer: manufacturer,
            year: year,
        }
    }

    // Getters
    fn manufacturer(&self) -> &'static str { self.manufacturer }
    fn model(&self) -> &'static str { self.model }
    fn year(&self) -> i32 { self.year }
    fn owned(&self) -> bool { self.owned}
    
    // Default trait methods can be overridden.
    fn noise(&self) -> &'static str {
        "deep vroom!"
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::vehicle::Vehicle;
    use crate::traits::vehicle::Car;

    #[test]
    fn impl_test() {
        let mut car: Car = Vehicle::new("Jaguar", "Formula E", 2023);
        car.purchase();
        car.noise();
    }
}
