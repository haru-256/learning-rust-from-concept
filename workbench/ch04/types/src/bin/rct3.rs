use std::rc::Rc;

#[derive(Debug)]
struct Node {
    data: i32,
    child: Option<Box<Node>>,
}

fn print_link(start_node: Box<Node>) {
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
        current_node = current_node.child.unwrap();
    }
}

fn main() {
    let mut node1 = Box::new(Node {
        data: 1,
        child: None,
    });
    let mut node2 = Box::new(Node {
        data: 2,
        child: None,
    });
    let node3 = Node {
        data: 3,
        child: None,
    };

    node1.child = Some(Box::new(node3));
    node2.child = Some(Box::new(node3));

    println!("link from node1");
    print_link(node1);
    println!("link from node2");
    print_link(node2);
}
