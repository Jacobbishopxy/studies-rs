#![feature(min_specialization)]

trait Visitor<T> {
    fn visit(&mut self, _visitable: &T);
}

// === IMPL ================================================================

struct B;

impl B {
    fn accept<V>(&self, visitor: &mut V)
    where
        V: Visitor<B>,
    {
        visitor.visit(self);
    }
}

struct A {
    child: B,
}

impl A {
    fn accept<V>(&self, visitor: &mut V)
    where
        V: Visitor<A> + Visitor<B>,
    {
        visitor.visit(self);
        self.child.accept(visitor);
    }
}

struct V;

impl<T> Visitor<T> for V {
    default fn visit(&mut self, _visitable: &T) {}
}

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
