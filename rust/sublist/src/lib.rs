#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

trait SliceCmp {
    type Item: PartialEq;
    fn sublist_of(self: &Self, slice: &[Self::Item]) -> bool;
    fn superlist_of(self: &Self, slice: &[Self::Item]) -> bool;
    fn equal(self: &Self, slice: &[Self::Item]) -> bool;
}

impl<Item: PartialEq> SliceCmp for [Item] {
    type Item = Item;

    fn sublist_of(self: &Self, slice: &[Self::Item]) -> bool {
        let len = self.len();
        if len == 0 {
            return true;
        }
        slice.windows(len)
             .any(|sub| sub == self)
    }

    fn superlist_of(self: &Self, slice: &[Self::Item]) -> bool {
        slice.sublist_of(self)
    }

    fn equal(self: &Self, slice: &[Self::Item]) -> bool {
        self.sublist_of(slice) && self.superlist_of(slice)
    }
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {

    let (is_sublist, is_superlist) = (
        _first_list.sublist_of(_second_list),
        _first_list.superlist_of(_second_list),
    );

    match (is_sublist, is_superlist) {
        (true, false) => {
            Comparison::Sublist
        },
        (false, true) => {
            Comparison::Superlist
        },
        (true, true) => {
            Comparison::Equal
        },
        (false, false) => {
            Comparison::Unequal
        }
    }
}
