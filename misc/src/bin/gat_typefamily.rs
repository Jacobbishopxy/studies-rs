#![feature(generic_associated_types)]
#![allow(incomplete_features)]

use std::{rc::Rc, sync::Arc};

// 用泛型关联类型支持类型家族（GAT 暂未稳定）

trait PointerFamily {
    type PointerType<T>;
}

#[derive(Debug)]
struct RcPointer;

impl PointerFamily for RcPointer {
    type PointerType<T> = Rc<T>;
}

#[derive(Debug)]

struct ArcPointer;

impl PointerFamily for ArcPointer {
    type PointerType<T> = Arc<T>;
}

struct MyDataStructure<T, PointerSel: PointerFamily> {
    data: PointerSel::PointerType<T>,
}

fn main() {
    let my_data_structure = MyDataStructure::<RcPointer, RcPointer> {
        data: Rc::new(RcPointer),
    };

    let x = my_data_structure.data;

    println!("{:?}", x);
}
