#![allow(dead_code, unused)]

use std::marker::PhantomData;
use std::rc::Rc;

fn main() {
    let a = BytesRepr::<u32>{
        bytes: 42u32.to_be_bytes().to_vec(),
        p: PhantomData
    };
}

struct BytesRepr<T> {
    bytes: Vec<u8>,
    p: PhantomData<T>,
}

struct WithLifetime<'a> {
    data: *const u8,
    p: PhantomData<&'a ()>,
}

struct NonSendSync {
    data: String,
    p: PhantomData<Rc<()>>,
}