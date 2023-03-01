use std::collections::HashMap;

type VirtualTable = HashMap<String, fn()>;

pub trait VirtualClass {
    fn get_virtual_table(&self) -> &VirtualTable;
}

#[macro_export]
macro_rules! virtual_class {
    ($class_name: ident { $($method_name: ident),+ } ) => {
        generate_virtual_table!{$class_name, { $($method_name),+ } }
        generate_util_functions!{$class_name, { $($method_name),+ } }
    };

    ($class_name: ident => $base_class: ident { $($method_name: ident),+ } ) => {
        generate_virtual_table!($class_name => $base_class { $($method_name),+ } );
        generate_util_functions!($class_name, { $($method_name),+ } );
    }
}

#[macro_export]
macro_rules! generate_virtual_table {
    ($class_name: ident, { $($method_name: ident),+ } ) => {
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        paste! {
            lazy_static! {
                static ref [<$class_name _vtable>]: VirtualTable = {
                    let mut table = VirtualTable::new();

                    $(
                    table.insert(
                        stringify!($method_name).into(), [<$class_name __ $method_name>]
                    );
                    )+

                    table
                };
            }
        }
    };

    ($class_name: ident => $base_class: ident { $($method_name: ident),+ } ) => {
        #[allow(non_snake_case)]
        #[allow(non_upper_case_globals)]
        paste! {
            lazy_static! {
                static ref [<$class_name _vtable>]: VirtualTable = {
                    let mut table = [<$base_class _vtable>].clone();

                    $(
                    table.insert(
                        stringify!($method_name).into(), [<$class_name __ $method_name>]
                    );
                    )+

                    table
                };
            }
        }
    }
}

#[macro_export]
macro_rules! generate_util_functions {
    ($class_name: ident, { $($method_name: ident),+ } ) => {
        struct $class_name {
            _virtual_table: &'static VirtualTable
        }

        paste! {
            impl $class_name {
                fn create_virtual() -> Self {
                    $class_name {
                        _virtual_table: &[<$class_name _vtable>]
                    }
                }
            }
        }


        paste! {
            $(
            #[allow(non_snake_case)]
            pub fn [<$class_name __ $method_name>]() {
                println!("{}::{}", stringify!($class_name), stringify!($method_name));
            }
            )+
        }

        impl VirtualClass for $class_name {
            fn get_virtual_table(&self) -> &VirtualTable {
                self._virtual_table
            }
        }
    }
}

pub fn _virtual_call<Class>(virtual_class: &Class, function_signature: &str)
where
    Class: VirtualClass,
{
    let table = virtual_class.get_virtual_table();
    table.get(function_signature).expect("There is no function")()
}

#[macro_export]
macro_rules! virtual_call {
    ($obj: ident, $function: ident) => {
        _virtual_call($obj, stringify!($function));
    };
}

