// aux-build:xc_private_method_lib.rs

extern crate xc_private_method_lib;

fn main() {
    let _ = xc_private_method_lib::Struct::static_meth_struct();
    //~^ ERROR: method `static_meth_struct` is private

    let _ = xc_private_method_lib::Enum::static_meth_enum();
    //~^ ERROR: method `static_meth_enum` is private
}
