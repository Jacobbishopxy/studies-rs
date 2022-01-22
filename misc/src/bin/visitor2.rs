#![feature(min_specialization)]

trait Visitor<T> {
    fn visit(&mut self, _visitable: &T);
}

impl<T, U: Visitable> Visitor<U> for T {
    default fn visit(&mut self, _visitable: &U) {}
}

trait Visitable {
    fn accept<V>(&self, visitor: &mut V)
    where
        Self: Sized,
    {
        visitor.visit(self);
    }
}

// === IMPL ================================================================

struct B;

impl Visitable for B {}

struct A {
    child: B,
}

impl Visitable for A {
    fn accept<V>(&self, visitor: &mut V) {
        visitor.visit(self);
        self.child.accept(visitor);
    }
}

struct V;

impl Visitor<A> for V {
    fn visit(&mut self, _visitable: &A) {
        eprintln!("A");
    }
}

impl Visitor<B> for V {
    fn visit(&mut self, _visitable: &B) {
        eprintln!("B");
    }
}

fn main() {
    let a = A { child: B };

    let mut v = V;

    a.accept(&mut v);
}
