mod virtual_table;

use crate::virtual_table::{
    VirtualClass, _virtual_call
};
use lazy_static::lazy_static;
use paste::paste;
use std::collections::HashMap;
use std::mem::transmute;

type VirtualTable = HashMap<String, fn()>;

virtual_class! {
    Base {
        only_base,
        both
    }
}

virtual_class! {
    Derived => Base {
        only_derived,
        both
    }
}

fn main() {
    let base = &*Box::new(Base::create_virtual());
    let derived = &*Box::new(Derived::create_virtual());

    let really_derived = unsafe { transmute::<&Derived, &Base>(derived) };

    virtual_call!(base, both);
    virtual_call!(really_derived, both);
    virtual_call!(really_derived, only_base);
    virtual_call!(really_derived, only_derived);
}
