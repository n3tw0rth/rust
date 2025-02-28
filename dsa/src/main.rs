use dsa::linkedlist::DoublyLinkedList;

fn main() {
    let mut doubly_linked_list: DoublyLinkedList<u8> = DoublyLinkedList::new();

    doubly_linked_list.push_front(5);
    doubly_linked_list.push_front(4);

    doubly_linked_list.pop_front();
    let value = doubly_linked_list.pop_front();
    println!("{:?}", value)
}
