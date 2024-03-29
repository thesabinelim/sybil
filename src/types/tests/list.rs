use crate::types::list::{list, List, ListOf, ListPat, NonEmptyList};

#[test]
fn describe_list_eq_it_returns_true_when_both_empty() {
    assert_eq!(list![], list![]);
}

#[test]
fn describe_list_eq_it_returns_true_on_match() {
    assert_eq!(list![1, true, "three"], list![1, true, "three"]);
}

#[test]
fn describe_list_eq_it_returns_false_on_mismatch() {
    assert_ne!(list![1, true, "three"], list![1, true, "mismatch"]);
}

#[test]
fn describe_list_spread_initialisation_it_works() {
    let list1 = list![1];
    let list2 = list!["three", '4'];
    let list3 = list![5.0];
    assert_eq!(
        list![..list1, true, ..list2, ..list3],
        list![1, true, "three", '4', 5.0]
    );
}

#[test]
fn describe_list_of_it_works() {
    type EmptyList = ListOf![];
    type List1 = ListOf![i32];
    type List2<'a> = ListOf![&'a str, char];
    type List3 = ListOf![f64];
    let _list: ListOf![
        ..EmptyList,
        ..List1,
        ..EmptyList,
        bool,
        ..List2,
        ..EmptyList,
        ..List3
    ] = list![1, true, "three", '4', 5.0];
}

#[test]
fn describe_list_len_it_works() {
    assert_eq!(list![1, true, "three"].len(), 3);
}

#[test]
fn describe_list_len_it_returns_zero_when_empty() {
    assert_eq!(list![].len(), 0);
}

#[test]
fn describe_list_pat_it_works() {
    let ListPat![item1, item2, ..rest] = list![1, true, "three", '4'];
    assert_eq!(item1, 1);
    assert_eq!(item2, true);
    assert_eq!(rest, list!["three", '4']);
}

#[test]
fn describe_list_pop_it_works() {
    assert_eq!(list![1, true, "three"].pop(), ("three", list![1, true]));
}

#[test]
fn describe_list_pop_it_works_with_only_one_item() {
    assert_eq!(list![1].pop(), (1, list![]));
}

#[test]
fn describe_list_append_it_works() {
    assert_eq!(
        list![1, true, "three"].append('4'),
        list![1, true, "three", '4']
    );
}

#[test]
fn describe_list_append_it_works_when_empty() {
    assert_eq!(list![].append(1), list![1]);
}

#[test]
fn describe_list_concat_it_works() {
    assert_eq!(
        list![1, true].concat(list!["three", '4']),
        list![1, true, "three", '4']
    );
}

#[test]
fn describe_list_concat_it_works_when_empty() {
    assert_eq!(
        list![].concat(list![1, true, "three"]),
        list![1, true, "three"]
    );
}

#[test]
fn describe_list_concat_it_works_when_both_empty() {
    assert_eq!(list![].concat(list![]), list![]);
}

#[test]
fn describe_list_reverse_it_works() {
    assert_eq!(list![1, true, "three"].reverse(), list!["three", true, 1]);
}

#[test]
fn describe_list_reverse_it_works_when_empty() {
    assert_eq!(list![], list![]);
}
