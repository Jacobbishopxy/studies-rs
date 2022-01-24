trait Visitor {
    fn visit_a(&mut self, _visitable: &A) {}
    fn visit_b(&mut self, _visitable: &B) {}
}

// === IMPL ================================================================

struct B;

impl B {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_b(self);
    }
}

struct A {
    child: B,
}

impl A {
    fn accept<V: Visitor>(&self, visitor: &mut V) {
        visitor.visit_a(self);
        self.child.accept(visitor);
    }
}

struct V;

impl Visitor for V {
    fn visit_a(&mut self, _visitable: &A) {
        eprintln!("A");
    }
    fn visit_b(&mut self, _visitable: &B) {
        eprintln!("B");
    }
}

fn main() {
    let a = A { child: B };

    let mut v = V;

    a.accept(&mut v);
}
