// FIXME: Make me pass! Diff budget: 10 lines.
// Do not `use` any items.

fn max<T: PartialOrd + PartialEq>(a: T, b: T) -> T {
    if a>b {
        a
    } else {
        b
    }
}

// Do not change the following two lines.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
struct IntWrapper(isize);

pub fn main() {
    assert_eq!(max(1usize, 3), 3);
    assert_eq!(max(1u8, 3), 3);
    assert_eq!(max(1u8, 3), 3);
    assert_eq!(max(IntWrapper(120).clone(), IntWrapper(248).clone()), IntWrapper(248));
}
