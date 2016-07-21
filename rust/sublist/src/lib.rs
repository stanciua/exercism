#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(list1: &[T], list2: &[T]) -> Comparison {
    if list1.len() == list2.len() {
        if list1 == list2 {
            return Comparison::Equal;
        } else {
            return Comparison::Unequal;
        }
    } else if list1.len() < list2.len() {
        let mut slice = &list2[..];
        while slice.len() >= list1.len() {
            if list1 == &slice[..list1.len()] {
                return Comparison::Sublist;
            }
            slice = &slice[1..];
        }
    } else {
        let mut slice = &list1[..];
        while slice.len() >= list2.len() {
            if list2 == &slice[..list2.len()] {
                return Comparison::Superlist;
            }
            slice = &slice[1..];

        }
    }

    Comparison::Unequal;
}
