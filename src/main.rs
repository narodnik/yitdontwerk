use std::sync::Arc;

struct Foo {}

impl Foo {
    fn new(x: u32) -> Arc<dyn Bar> {
        Arc::new(Self {})
    }
}

trait Bar {}

impl Bar for Foo {}

struct Blaa<'b> {
    f: Vec<Box<dyn 'b + Fn(u32) -> Arc<dyn Bar>>>,
    _phantom: std::marker::PhantomData<&'b u32>
}

impl<'b> Blaa<'b> {
    fn new() -> Self {
        Self {
            f: Vec::new(),
            _phantom: std::marker::PhantomData
        }
    }

    fn register<C>(&mut self, construct: C)
    where
         C: 'b + Fn(u32) -> Arc<dyn Bar>,
    {
        //This does not work
        self.f.push(Box::new(construct));
    }
}


struct Other {
}
impl Other {
    fn new(x: u32, athing: String) -> Arc<dyn Bar> {
        Arc::new(Self {})
    }
}
impl Bar for Other {
}

fn main() {
    let mut b = Blaa::new();
    b.register(|x| Other::new(x, "sdds".to_string()));
    b.register(Foo::new);
}
