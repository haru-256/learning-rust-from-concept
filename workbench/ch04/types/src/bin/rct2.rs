use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    data: i32,
    child: Option<Rc<RefCell<Node>>>,
}

fn print_link(start_node: Rc<RefCell<Node>>) {
    let mut current_node = start_node;
    loop {
        println!("{}", current_node.borrow().data);
        // match current_node.child {
        //     Some(ref next_node) => current_node = Rc::clone(next_node),
        //     None => break,
        // }
        if current_node.borrow().child.is_none() {
            break;
        }
        let tmp = Rc::clone(current_node.borrow().child.as_ref().unwrap());
        current_node = tmp;
    }
}

fn main() {
    let node1 = Rc::new(RefCell::new(Node {
        data: 1,
        child: None,
    }));
    let node2 = Rc::new(RefCell::new(Node {
        data: 2,
        child: None,
    }));
    let node3 = Rc::new(RefCell::new(Node {
        data: 3,
        child: None,
    }));

    node1.borrow_mut().child = Some(Rc::clone(&node3));
    node1.borrow_mut().child = Some(Rc::clone(&node3));

    println!("link from node1");
    print_link(Rc::clone(&node1));
    println!("link from node2");
    print_link(Rc::clone(&node2));
}
