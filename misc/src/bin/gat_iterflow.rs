#![feature(generic_associated_types)]
#![allow(incomplete_features)]

// 用泛型关联类型实现流式处理迭代器

trait FunctorFamily {
    type Type<T>;

    fn fmap<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> U;
}

trait ApplicativeFamily: FunctorFamily {
    fn pure<T>(inner: T) -> Self::Type<T>;

    fn apply<T, U, F>(value: Self::Type<T>, f: Self::Type<F>) -> Self::Type<U>
    where
        F: FnMut(T) -> U;
}

trait MonadFamily: ApplicativeFamily {
    fn bind<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> Self::Type<U>;
}

// 为一个“选择器”实现这些类型：

struct OptionType;

impl FunctorFamily for OptionType {
    type Type<T> = Option<T>;

    fn fmap<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> U,
    {
        value.map(f)
    }
}

impl ApplicativeFamily for OptionType {
    fn pure<T>(inner: T) -> Self::Type<T> {
        Some(inner)
    }

    fn apply<T, U, F>(value: Self::Type<T>, f: Self::Type<F>) -> Self::Type<U>
    where
        F: FnMut(T) -> U,
    {
        value.zip(f).map(|(v, mut f)| f(v))
    }
}

impl MonadFamily for OptionType {
    fn bind<T, U, F>(value: Self::Type<T>, f: F) -> Self::Type<U>
    where
        F: FnMut(T) -> Self::Type<U>,
    {
        value.and_then(f)
    }
}

fn main() {
    let ot = Some(233);

    let f1 = |i: i32| format!("> {} <", i);
    let f2 = |i: Option<i32>| Some(i);

    let foo = OptionType::fmap(ot, f1);
    let bar = OptionType::pure(ot);
    let qux = OptionType::apply(ot, Some(f1));
    let quz = OptionType::pure(ot).and_then(f2);

    println!("{:?}", foo);
    println!("{:?}", bar);
    println!("{:?}", qux);
    println!("{:?}", quz);
}
