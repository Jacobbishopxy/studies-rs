use core::any::{Any, TypeId};
use std::ops::Deref;

struct Rectangle;
struct Triangle;

trait Shape: Any {}

// 为 Rectangle 与 Triangle 都实现 Shape 特性
impl Shape for Rectangle {}
impl Shape for Triangle {}

fn main() {
    let shapes: Vec<Box<dyn Shape>> =
        vec![Box::new(Rectangle), Box::new(Triangle), Box::new(Rectangle)];

    let n = count_rectangles(&shapes);
    assert_eq!(2, n);

    let mut shapes: Vec<Box<dyn Any>> =
        vec![Box::new(Rectangle), Box::new(Triangle), Box::new(Rectangle)];

    remove_first_rectangle(&mut shapes).expect("No rectangle found to be removed");

    assert_eq!(2, shapes.len());
}

fn count_rectangles(shapes: &[Box<dyn Shape>]) -> usize {
    let mut n = 0;
    for shape in shapes {
        // 需要对 shape 进行解引用，否则将会拿到 Box 的 type_id
        let type_of_shape = shape.deref().type_id();
        if type_of_shape == TypeId::of::<Rectangle>() {
            n += 1;
        } else {
            println!("{:?} is not a Rectangle", type_of_shape)
        }
    }
    n
}

fn remove_first_rectangle(shapes: &mut Vec<Box<dyn Any>>) -> Option<Box<Rectangle>> {
    let idx = shapes
        .iter()
        .position(|shape| shape.deref().type_id() == TypeId::of::<Rectangle>())?;

    let rectangle_as_unknown_shape = shapes.remove(idx);
    rectangle_as_unknown_shape.downcast().ok()
}
