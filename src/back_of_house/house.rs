pub struct BreakFast {
    pub toast: String,
    seasonal_fruit: String,
}

impl BreakFast {
    pub fn summer(toast: &str) -> BreakFast {
        BreakFast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

 
