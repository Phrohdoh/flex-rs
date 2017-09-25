extern crate flex;
use flex::FlexItem;

fn main() {
    let mut root = FlexItem::new();
    let mut child = FlexItem::new();
    let grandchild = FlexItem::new();

    println!("g: {:?}", grandchild);
    child.add_child(grandchild);

    println!("c: {:?}", child);
    root.add_child(child);

    println!("r: {:?}", root);
}