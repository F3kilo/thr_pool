use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

fn main() {
    // Структуры с `Send + Sync` полями `Send + Sync`.
    assert_send::<BothStruct>();
    assert_sync::<BothStruct>();

    // `RefCell: Send+!Sync`
    assert_send::<RefCell<()>>();
    assert_sync::<RefCell<()>>();

    // `&RefCell: !Send+!Sync`
    assert_send::<&RefCell<()>>();
    assert_sync::<&RefCell<()>>();

    // `Rc: !Send+!Sync`
    assert_send::<Rc<()>>();
    assert_sync::<Rc<()>>();

    // Структуры с `!Send + !Sync` полями `!Send + !Sync`.
    assert_send::<NeitherStruct>();
    assert_sync::<NeitherStruct>();
}

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

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
