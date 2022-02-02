use async_std::sync::Mutex;
use std::sync::Arc;
use futures::future::BoxFuture;
use std::future::Future;

type ProtocolBasePtr = Arc<dyn ProtocolBase>;

trait ProtocolBase {
}

// Specialization of ProtocolBase trait
struct ProtocolPing {
}
impl ProtocolPing {
    fn new(x: u8) -> Arc<dyn ProtocolBase> {
        Arc::new(Self {})
    }
}
impl ProtocolBase for ProtocolPing {
}

// You cannot change this. This is fine.
type Constructor = Box<dyn Fn(u32) -> BoxFuture<'static, ProtocolBasePtr> + Send + Sync>;

struct ProtocolRegistry {
    protocol_constructors: Vec<Constructor>
}

impl ProtocolRegistry {
    // This makes no sense.
    async fn register<C, F>(&mut self, constructor: C)
    where
        C: 'static + Fn(u32) -> F + Send + Sync,
        F: 'static + Future<Output=Arc<dyn ProtocolBase + 'static>> + Send
    {
        let constructor = Box::new(move |x| Box::pin(constructor(x)));
        //self.protocol_constructors.push(constructor);
    }
}

// From the stackoverflow examples
struct S {
    //foo: Box<dyn Fn(u8) -> BoxFuture<'static, u8> + Send + Sync>,
    foo: Box<dyn Fn(u8) -> BoxFuture<'static, Arc<dyn ProtocolBase>> + Send + Sync>,
}

fn foo(x: u8) -> BoxFuture<'static, Arc<dyn ProtocolBase>> {
    Box::pin(async move { ProtocolPing::new(x) })
}

async fn foo_simple(x: u8) -> u8 {
    x * 2
}

fn main() {
    // This works
    let s = S { foo: Box::new(foo) };
    // This also
    //let s = S { foo: Box::new(move |x| Box::pin(foo_simple(x))) };
}

