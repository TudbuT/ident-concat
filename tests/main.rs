use ident_concat::*;
fn main() {
    let ident!(a b) = 4;
    assert_eq!(ab, ident!(a b));
    replace! { placeholder te st:
        fn placeholder() -> u32 { 4 }
    }
    assert_eq!(test(), 4);
}
