#[macro_use]
extern crate tt_call;

tt_call! {
    macro = [{ parse_type }]
    input = [{ <T as @ }]
}

fn main() {}
