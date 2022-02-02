use std::future::Future;
use futures::future::BoxFuture; // Pin<Box<dyn Future<Output = T> + Send>>

struct X {
}

async fn test() -> X {
    X{}
}

//fn reg(f: Box<dyn Fn() -> Future<Output=X>>) {
//}

struct S {
    foo: Box<dyn Fn() -> BoxFuture<'static, X>>,
}

fn foo() -> BoxFuture<'static, X> {
    Box::pin(async {
        test().await
    })
}

async fn foof() -> X {
    X{}
}

fn main() {
    let s = S { foo: Box::new(move || Box::pin(foof())) };
    //let s = S { foo: Box::new(foo) };
}

