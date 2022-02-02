use async_std::sync::Mutex;
use std::sync::Arc;
use futures::future::BoxFuture;
use std::future::Future;

type ProtocolBasePtr = Arc<dyn ProtocolBase>;

trait ProtocolBase {
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
        self.protocol_constructors.push(constructor);
    }
}

fn main() {
}

