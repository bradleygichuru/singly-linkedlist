use std::ptr::NonNull;

struct Node<T> {
    data: T,
    next: Option<NonNull<Node<T>>>,
    is_head: bool,
}
#[derive(Debug)]
struct SinglyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
}
impl<T: std::cmp::PartialEq + std::fmt::Debug> SinglyLinkedList<T> {
    fn new() -> SinglyLinkedList<T> {
        SinglyLinkedList { head: None }
    }
    fn insert_beginning(&mut self, data: T) -> bool {
        //insert a node at the beginning of the list
        if self.head.is_none() {
            let new_node = Box::new(Node {
                data: data,
                next: None,
                is_head: true,
            });

            let head_ptr = Box::leak(new_node).into();
            self.head = Some(head_ptr);
            true
        } else {
            let new_node = Box::new(Node {
                data: data,
                next: self.head,
                is_head: true,
            });

            let head_ptr = Box::leak(new_node).into();
            self.head = Some(head_ptr);
            true
        }
    }
    fn insert_end(&mut self, data: T) -> bool { //TODO test
        //insert a node at the end of the list
        // let current_node = &self.head;
        if self.head.is_some() {
            let mut current_node = Box::new(self.head);

            let new_node = Box::new(Node {
                data: data,
                next: None,
                is_head: false,
            });
            let new_node_pointer = Box::leak(new_node).into();
            let mut current_node_ptr: &mut Option<NonNull<Node<T>>> =
                Box::leak(current_node).into();
            unsafe {
                while current_node_ptr.unwrap().as_mut().next.is_some() {
                    current_node = Box::new(current_node_ptr.unwrap().as_mut().next);

                    current_node_ptr = Box::leak(current_node).into();
                }
                current_node_ptr.unwrap().as_mut().next = Some(new_node_pointer);
                true
            }
        } else {
            println!("Linked list has no head");
            false
        }
    }
    fn delete_node(&mut self, data: T) {//TODO test
        if self.head.is_some() {
            let mut current_node = Box::new(self.head);
            let mut prev_node = Box::new(self.head);
            let mut prev_node_ptr: &mut Option<NonNull<Node<T>>> = Box::leak(prev_node).into();
            let mut current_node_ptr: &mut Option<NonNull<Node<T>>> =
                Box::leak(current_node).into();
            unsafe {
                while current_node_ptr.unwrap().as_mut().next.is_some() {
                    if current_node_ptr.unwrap().as_mut().data == data {
                        //if current node is our target node
                        if prev_node_ptr.unwrap().as_mut().is_head == false {
                            //if not the head node , unlink the current node by setting the previous
                            //node's next pointer to the current node's next pointer
                            //pointer to the previous's node next node
                            prev_node_ptr.unwrap().as_mut().next =
                                current_node_ptr.unwrap().as_mut().next;
                            break;
                        } else {
                            current_node_ptr
                                .unwrap()
                                .as_mut()
                                .next
                                .unwrap()
                                .as_mut()
                                .is_head = true;
                            self.head = *current_node_ptr;
                        }
                    } else {
                        prev_node = Box::new(*current_node_ptr);
                        prev_node_ptr = Box::leak(prev_node);
                        current_node = Box::new(current_node_ptr.unwrap().as_mut().next);

                        current_node_ptr = Box::leak(current_node).into();
                    }
                }
            }
        } else {
            println!("Linked list has no head");
        }
    }
    // fn search(data: T) -> {}
    fn traverse(&mut self) {
        if self.head.is_some() {
            let mut current_node = Box::new(self.head);

            let mut current_node_ptr: &mut Option<NonNull<Node<T>>> =
                Box::leak(current_node).into();
            unsafe {
                /* while current_node_ptr.unwrap().as_mut().next.is_some() {
                    println!("data : {:?}",current_node_ptr.unwrap().as_mut().data);
                    current_node = Box::new(current_node_ptr.unwrap().as_mut().next);

                    current_node_ptr = Box::leak(current_node).into();
                } */
                loop {
                    /* println!(
                        "data : {:?} ,next: {:?}",
                        current_node_ptr.unwrap().as_mut().data,
                        current_node_ptr
                            .unwrap()
                            .as_mut()
                            .next
                            .unwrap()
                            .as_mut()
                            .data
                    ); */
                    if current_node_ptr.unwrap().as_mut().next.is_none() {
                        println!("data: {:?}", current_node_ptr.unwrap().as_mut().data);

                        break;
                    }

                    if current_node_ptr.unwrap().as_mut().next.is_some() {
                        println!("data: {:?}", current_node_ptr.unwrap().as_mut().data);

                        current_node = Box::new(current_node_ptr.unwrap().as_mut().next);

                        current_node_ptr = Box::leak(current_node).into();
                    }
                }
            }
        } else {
            println!("Linked list has no head");
        }
    }
}
fn main() {
    println!("Hello, world!");
}
#[test]
fn test_insert() {
    let mut ln = SinglyLinkedList::new();

    ln.insert_beginning(1);
    ln.insert_beginning(2);

    ln.insert_beginning(3);

    ln.insert_beginning(4);
    ln.insert_beginning(5);
    ln.insert_beginning(6);
    ln.insert_beginning(7);
    ln.insert_beginning(8);
    ln.traverse();
    //ln.delete_node(1);

    unsafe {
        assert_eq!(ln.head.unwrap().as_mut().data, 8);
        println!("head : {:?}", ln.head.unwrap().as_mut().data);
    }
}
