use std::{marker::PhantomData, sync::MutexGuard};

use koto::{derive::*, prelude::*, Result};

fn main() {
    let script = "
my_type = make_my_type 41
print my_type.get()
";
    let mut koto = Koto::default();

    koto.prelude()
        .add_fn("make_my_type", |ctx| match ctx.args() {
            [KValue::Number(n)] => Ok(MyType::make_koto_object(*n).into()),
            unexpected => type_error_with_slice("a number", unexpected),
        });

    koto.compile_and_run(script).unwrap();
}

// Using MutexGuard to ensure that the type isn't Sendable
#[derive(Clone, Copy, KotoCopy, KotoType)]
struct MyType(i64, PhantomData<MutexGuard<'static, ()>>);

#[koto_impl]
impl MyType {
    fn make_koto_object(n: KNumber) -> KObject {
        let my_type = Self(n.into(), Default::default());
        KObject::from(my_type)
    }

    #[koto_method]
    fn get(&self) -> Result<KValue> {
        Ok(self.0.into())
    }
}

impl KotoObject for MyType {
    fn display(&self, ctx: &mut DisplayContext) -> Result<()> {
        ctx.append(format!("MyType({})", self.0));
        Ok(())
    }
}
