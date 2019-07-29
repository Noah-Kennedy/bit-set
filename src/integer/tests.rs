use super::*;

fn bounded_unchecked_insertion<S, T>(value: u64) -> bool where S: IntegerBitSet<T> {
    let mut set = S::default();

    set.insert_unchecked(value);

    if set.in_bounds(value) {
        set.contains_unchecked(value)
    } else {
        let data = set.try_into().unwrap();
        println!("Value: {}", value);
        println!("Result: {:x?}", data);
        data == 0
    }
}

fn bounded_unchecked_contain<S, T>(value: u64) -> bool where S: IntegerBitSet<T> {
    let mut set = S::default();

    set.insert_unchecked(value);

    if set.in_bounds(value) {
        set.contains_unchecked(value)
    } else {
        !set.contains_unchecked(value)
    }
}

#[quickcheck]
fn bounded_unchecked_insertion_u16(value: u64) -> bool {
    bounded_unchecked_insertion::<IntegerBitSet16, u16>(value)
}

#[quickcheck]
fn bounded_unchecked_contains_u16(value: u64) -> bool {
    bounded_unchecked_contain::<IntegerBitSet16, u16>(value)
}