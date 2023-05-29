use ident_concat::*;
fn main() {
    let ident!(a b): u32 = 4;
    assert_eq!(ab, ident!(a b));
    replace! { placeholder te st:
        fn placeholder() -> u32 { 4 }
        assert_eq!(test(), 4);
    }
    replace! { placeholder a b c:
        fn placeholder(abc: u32) -> u32 { placeholder }
        assert_eq!(ab, abc(ab));
    }
}
