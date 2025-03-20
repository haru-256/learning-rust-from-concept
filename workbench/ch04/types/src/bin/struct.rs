fn main() {
    #[derive(Debug)]
    pub struct Person {
        pub name: String,
        pub age: u8,
    }

    impl Person {
        // associated function
        pub fn new(name: String, age: u8) -> Person {
            Person { name, age }
        }

        // method
        pub fn age_incr(&self, incr: u8) -> u8 {
            self.age + incr
        }

        // method
        pub fn age_incr_replace(&mut self, incr: u8) {
            self.age += incr;
        }
    }

    let taro = Person {
        name: String::from("Taro"),
        age: 20,
    };
    println!("{} is {} years old.", taro.name, taro.age);
    println!("{:?}", taro);

    let mut taro = Person::new(String::from("Taro"), 20);
    taro.age = 22;
    println!("{} is {} years old.", taro.name, taro.age);

    let taro = &mut Person::new(String::from("Taro"), 20);
    taro.age = 21;
    println!("{} is {} years old.", taro.name, taro.age);

    let name = String::from("Jiro");
    let age = 25;
    let jiro = Person { name, age };
    println!("{} is {} years old.", jiro.name, jiro.age);

    let mut hana = Person {
        name: String::from("Hana"),
        ..jiro
    };
    println!("{} is {} years old.", hana.name, hana.age);

    let age_ = hana.age_incr(7);
    println!("{} is {} years old.", hana.name, age_);
    hana.age_incr_replace(3);
    println!("{} is {} years old.", hana.name, age_);

    struct Color(u8, u8, u8);
    let color = Color(255, 0, 0);
    println!("R: {}, G: {}, B: {}", color.0, color.1, color.2);

    #[derive(Debug)]
    struct Parents<'a, 'b> {
        father: &'a mut Person,
        mother: &'b mut Person,
    }
    impl<'a, 'b> Parents<'a, 'b> {
        fn new(father: &'a mut Person, mother: &'b mut Person) -> Parents<'a, 'b> {
            Parents { father, mother }
        }

        pub fn age_incr(&self, incr: u8) -> u8 {
            self.father.age + incr
        }

        pub fn age_incr_replace(&mut self, incr: u8) {
            self.father.age += incr;
            self.mother.age += incr;
        }
    }

    let mut taro = Person::new(String::from("Taro"), 20);
    let mut hanako = Person::new(String::from("Hanako"), 18);
    let mut sato = Parents::new(&mut taro, &mut hanako);
    println!("{:?}", sato);
    println!("{:?}", sato.father);
    println!("{:?}", sato.mother);
    sato.age_incr(5);
    sato.age_incr_replace(5);
    println!("{:?}", sato.father);
    println!("{:?}", sato.mother);
}
