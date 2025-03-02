use dsa::linkedlist::DoublyLinkedList;

fn main() {
    let mut doubly_linked_list: DoublyLinkedList<u8> = DoublyLinkedList::new();

    println!("Is emtpy: {}", doubly_linked_list.is_emtpy());

    doubly_linked_list.push_front(5);
    doubly_linked_list.push_front(4);
    doubly_linked_list.push_back(6);

    doubly_linked_list.traverse_back();
    doubly_linked_list.traverse_front();

    println!("Is emtpy: {}", doubly_linked_list.is_emtpy());
}
