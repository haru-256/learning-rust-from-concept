trait SayHello {
    fn say_hello(&self);
}

trait SayThankYou {
    fn say_thank_you(&self);
}

trait Run {
    fn run(&self) {
        println!("Running!");
    }
}

#[derive(Debug)]
struct EnglishPerson {
    name: String,
}
impl EnglishPerson {
    fn new(name: &str) -> Self {
        EnglishPerson {
            name: String::from(name),
        }
    }
}
impl SayHello for EnglishPerson {
    fn say_hello(&self) {
        println!("Hello, {}!", self.name);
    }
}
impl SayThankYou for EnglishPerson {
    fn say_thank_you(&self) {
        println!("Thank you, {}!", self.name);
    }
}
impl Run for EnglishPerson {
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

#[derive(Debug)]
struct JapanesePerson {
    name: String,
}
impl JapanesePerson {
    fn new(name: &str) -> Self {
        JapanesePerson {
            name: String::from(name),
        }
    }
}
impl SayHello for JapanesePerson {
    fn say_hello(&self) {
        println!("こんにちは, {}!", self.name);
    }
}
impl SayThankYou for JapanesePerson {
    fn say_thank_you(&self) {
        println!("ありがとう, {}!", self.name);
    }
}
impl Run for JapanesePerson {
    fn run(&self) {
        println!("{} は走っている!", self.name);
    }
}

fn say_hello<T: SayHello>(person: &T) {
    person.say_hello();
}
fn say_thank_you<T: SayThankYou>(person: &T) {
    person.say_thank_you();
}
fn say_thank_you_and_run<T: SayThankYou + Run>(person: &T) {
    person.say_thank_you();
    person.run();
}

fn main() {
    let hanako = JapanesePerson::new("花子");
    let john = EnglishPerson::new("John");

    println!("Japanese: {:?}", hanako);
    say_hello(&hanako);
    say_thank_you(&hanako);
    say_thank_you_and_run(&hanako);

    println!("EnglishPerson: {:?}", john);
    say_hello(&john);
    say_thank_you(&john);
    say_thank_you_and_run(&john);
}
