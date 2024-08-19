use std::rc::Rc;

#[derive(Debug)]
struct Node {
    data: i32,
    child: Option<Rc<Node>>,
}

fn print_link(start_node: Rc<Node>) {
    let mut current_node = start_node;
    loop {
        println!("{}", current_node.data);
        // match current_node.child {
        //     Some(ref next_node) => current_node = Rc::clone(next_node),
        //     None => break,
        // }
        if current_node.child.is_none() {
            break;
        }
        current_node = Rc::clone(current_node.child.as_ref().unwrap());
    }
}

fn main() {
    let node3 = Rc::new(Node {
        data: 3,
        child: None,
    });

    let node1 = Rc::new(Node {
        data: 1,
        child: Some(Rc::clone(&node3)),
    });
    let node2 = Rc::new(Node {
        data: 1,
        child: Some(Rc::clone(&node3)),
    });

    println!("link from node1");
    print_link(Rc::clone(&node1));
    println!("link from node2");
    print_link(Rc::clone(&node2));
}
