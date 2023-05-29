#![allow(dead_code, unused)]

use std::marker::PhantomData;

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