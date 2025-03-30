use std::ops::Deref;

#[derive(Debug)]
struct MyData {
    x: i32,
}

impl Deref for MyData {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.x
    }
}

fn deref_data(x: &i32) -> i32 {
    *x
}

fn main() {
    let data = MyData { x: 42 };
    let data2 = &data;
    let data_deref1 = data.deref();
    let data_deref2 = *(data.deref());
    let data_deref3 = *data;
    let data_deref4 = deref_data(&data);

    println!("data = {:?}", data);
    println!("data2 = {:?}", data2);
    println!("data_deref1 = {:?}", data_deref1);
    println!("data_deref2 = {:?}", data_deref2);
    println!("data_deref3 = {:?}", data_deref3);
    println!("data_deref4 = {:?}", data_deref4);
}
