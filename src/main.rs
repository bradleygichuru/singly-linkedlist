use std::ptr::NonNull;

struct Node<T> {
    data: T,
    next: Option<NonNull<Node<T>>>,
}
struct SinglyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
}
impl<T> SinglyLinkedList<T> {
     fn new()->SinglyLinkedList<T>{
        SinglyLinkedList { head: None}


    } 
    fn insert_beginning(&mut self, data: T) -> bool {
        //insert a node at the beginning of the list
        if self.head.is_none() {
            let new_node = Box::new(Node {
                data: data,
                next: None,
            });

            let head_ptr = Box::leak(new_node).into();
            self.head = Some(head_ptr);
            true
        } else {
            let new_node = Box::new(Node {
                data: data,
                next: self.head,
            });

            let head_ptr = Box::leak(new_node).into();
            self.head = Some(head_ptr);
            true
        }
    }
    fn insert_end(self, data: T) -> bool {
        //insert a node at the end of the list
        // let current_node = &self.head;
        if self.head.is_some() {
            let mut current_node = Box::new(self.head);
            
            let new_node = Box::new(Node {
                data: data,
                next: None,
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
            false
        }
    }
}
fn main() {
    println!("Hello, world!");
}
#[test]
fn test_insert(){
    let mut ln = SinglyLinkedList::new();
    ln.insert_beginning(1);
    ln.insert_beginning(2);
    unsafe{
        assert_eq!(ln.head.unwrap().as_mut().data,2);
    println!("head {:?}",ln.head.unwrap().as_mut().data);

    }
}
