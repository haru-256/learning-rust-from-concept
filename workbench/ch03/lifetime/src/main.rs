fn pick1(x: &[i32], end: usize) -> &[i32] {
    &x[..end]
}

fn pick2<'a, 'b>(x: &'a [i32], y: &'b [i32], end: usize) -> (&'a [i32], &'b [i32]) {
    (&x[..end], &y[..end])
}

fn pick3<'a>(x: &'a [i32], y: &'a [i32], end: usize) -> (&'a [i32], &'a [i32]) {
    (&x[..end], &y[..end])
}

fn main() {
    let v1 = [1, 2, 3, 4, 5];
    let p = pick1(&v1, 2);
    for ss in p {
        println!("{}", ss);
    }

    let v1 = [1, 2, 3, 4, 5];
    let v2 = [6, 7, 8, 9, 10];
    let p = pick2(&v1, &v2, 2);
    for ss in p.0 {
        println!("{}", ss);
    }
    for ss in p.1 {
        println!("{}", ss);
    }

    let v1 = [1, 2, 3, 4, 5];
    let v2 = [6, 7, 8, 9, 10];
    let p = pick3(&v1, &v2, 2);
    for ss in p.0 {
        println!("{}", ss);
    }
    for ss in p.1 {
        println!("{}", ss);
    }
}
