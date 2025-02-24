use dsa::linkedlist::singly::LinkedList;

fn main() {
    let mut linked_list: LinkedList<String> = LinkedList::new();

    linked_list.push("a".to_string());
    linked_list.push("b".to_string());

    let poped_value = linked_list.pop();

    println!("{:?}", poped_value);
    println!("{:?}", linked_list.is_empty())
}
