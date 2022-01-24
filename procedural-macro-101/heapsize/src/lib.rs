//! HeapSize
//!
//! https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize/src/lib.rs

pub use heapsize_derive::*;

pub trait HeapSize {
    /// 为 `self` 所拥有的的堆的总字节大小
    fn heap_size_of_children(&self) -> usize;
}

// 在现实世界中应该要有更多的实现，但是这里只是为了示例
impl HeapSize for u8 {
    /// u8 的堆大小为 0
    fn heap_size_of_children(&self) -> usize {
        0
    }
}

impl HeapSize for String {
    /// 一个 `String` 拥有足够的堆内存来容纳它的预留容量。
    fn heap_size_of_children(&self) -> usize {
        self.capacity()
    }
}

impl<T> HeapSize for Box<T>
where
    T: ?Sized + HeapSize,
{
    /// 一个 `Box` 的值被分配多少堆内存是取决于 `T` 位于堆上的大小
    /// 以及其自身所拥有的 T 的大小
    fn heap_size_of_children(&self) -> usize {
        std::mem::size_of_val(&**self) + (**self).heap_size_of_children()
    }
}

impl<T> HeapSize for [T]
where
    T: HeapSize,
{
    /// 动态大小
    fn heap_size_of_children(&self) -> usize {
        self.iter().map(|x| x.heap_size_of_children()).sum()
    }
}

impl<'a, T> HeapSize for &'a T
where
    T: ?Sized,
{
    /// 引用值的堆大小为 0
    fn heap_size_of_children(&self) -> usize {
        0
    }
}
