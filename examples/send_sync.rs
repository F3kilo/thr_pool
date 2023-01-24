use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

fn main() {
    // Структуры с `Send + Sync` полями `Send + Sync`.
    assert_send(BothStruct::default());
    assert_sync(BothStruct::default());

    // `RefCell: Send+!Sync`
    assert_send(RefCell::<()>::default());
    assert_sync(RefCell::<()>::default());

    // `&RefCell: Send+!Sync`
    assert_send(&RefCell::<()>::default());
    assert_sync(&RefCell::<()>::default());

    // `Rc: !Send+!Sync`
    assert_send(Rc::<()>::default());
    assert_sync(Rc::<()>::default());

    // Структуры с `!Send + !Sync` полями `!Send + !Sync`.
    assert_send(NeitherStruct::default());
    assert_sync(NeitherStruct::default());
}

fn assert_send<T: Send>(_: T) {}
fn assert_sync<T: Sync>(_: T) {}

#[derive(Default)]
struct BothStruct {
    _number: u32,
    _array: [u8; 10],
    _string: String,
    _map: HashMap<u64, Vec<u8>>,
}

#[derive(Default)]
struct NeitherStruct {
    _p: PhantomData<Rc<()>>,
}
