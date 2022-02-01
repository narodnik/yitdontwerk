use std::{future::Future, sync::Arc};

struct Foo {}

impl Foo {
    async fn new(x: u32) -> Arc<dyn Bar> {
        Arc::new(Self {})
    }
}

trait Bar {}

impl Bar for Foo {}

struct Blaa<'b> {
    f: Vec<Box<dyn 'b + Fn(u32) -> Future<Output=Arc<dyn Bar>>>>,
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
         'a: 'b
    {
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
