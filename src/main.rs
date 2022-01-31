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
    f: Vec<Box<dyn Fn(u32) -> Arc<dyn Bar>>>,
    _phantom: std::marker::PhantomData<&'b u32>
}

impl<'b> Blaa<'b> {
    fn new() -> Self {
        Self {
            f: Vec::new(),
            _phantom: std::marker::PhantomData
        }
    }

    fn register<'a, C>(&mut self, construct: C)
    where
         C: 'a + Fn(u32) -> Arc<dyn Bar>,
         'b: 'a
    {
        //This does not work
        //self.f.push(Box::new(construct));
    }

    // This works
    fn xx(&mut self, x: Box<dyn Fn(u32) -> Arc<dyn Bar>>) {
        self.f.push(x);
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
    //b.register(|x| Other::new(x, "sdds".to_string()));
    //b.register(Foo::new);
    b.xx(Box::new(Foo::new));
}
