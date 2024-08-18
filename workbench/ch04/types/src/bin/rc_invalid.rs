use std::rc::Rc;

struct DataA<'a> {
    number_a: Option<&'a i32>,
}

struct DataB<'a> {
    number_b: Option<&'a i32>,
}

fn setdata(data_a: &mut DataA, data_b: &mut DataB, value: i32) {
    let number = Box::new(value + 1);
    data_a.number_a = Some(&number);
    data_b.number_b = Some(&number);
}

fn main() {
    let mut data_a1 = DataA { number_a: None };
    let mut data_b1 = DataB { number_b: None };
    let mut data_a2 = DataA { number_a: None };
    let mut data_b2 = DataB { number_b: None };

    setdata(&mut data_a1, &mut data_b1, 10);
    println!(
        "to be 11, 11: {}, {}",
        data_a1.number_a.unwrap(),
        data_b1.number_b.unwrap()
    );

    setdata(&mut data_a2, &mut data_b2, 20);
    println!(
        "to be 21, 21: {}, {}",
        data_a2.number_a.unwrap(),
        data_b2.number_b.unwrap()
    );
}
