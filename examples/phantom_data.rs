use std::marker::PhantomData;

fn main() {
    let _a = BytesRepr::<u32>{
        _bytes: 42u32.to_be_bytes().to_vec(),
        _p: PhantomData
    };
}

struct BytesRepr<T> {
    _bytes: Vec<u8>,
    _p: PhantomData<T>,
}