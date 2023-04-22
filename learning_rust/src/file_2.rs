mod Person {
    pub struct Personal_info {
        pub age: u8,
        pub education: String,
    }

    impl Personal_info {
        pub fn new(new_edu: &str) -> Self {
            Self {
                education: String::from(new_edu),
                age: 25,
            }
        }
    }
}

pub fn some_person() {
    let mut person1 = Person::Personal_info::new("Undergrad");
    person1.education = String::from("Undergrad");

    let person2 = Person::Personal_info {
        age: 25,
        education: String::from("Undergrad"),
    };
}
