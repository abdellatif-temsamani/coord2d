use crate::*;

#[test]
fn new_vector() {
    let start: coord::Coord = new_coord!(2.5, 3.3);
    let end: coord::Coord = new_coord!(2.5);

    let vec: vector::Vector = new_vector!(start, end);
    assert_eq!(vec.start, start);
    assert_eq!(vec.end, end);

    let vec: vector::Vector = new_vector!(end);
    assert_eq!(vec.start, new_coord!());
    assert_eq!(vec.end, end);

    let vec: vector::Vector = new_vector!();

    assert_eq!(vec.start, new_coord!());
    assert_eq!(vec.end, new_coord!());
}

#[test]
fn to_vec() {
    let start: coord::Coord = new_coord!(2.5, 3.3);
    let end: coord::Coord = new_coord!(2.5);

    let vec: vector::Vector = new_vector!(start, end);
    let g = vec.to_vec();
    assert_eq!(g[0], start);
    assert_eq!(g[1], end);
}

#[test]
fn to_tuple() {
    let start: coord::Coord = new_coord!(2.5, 3.3);
    let end: coord::Coord = new_coord!(2.5);

    let vec: vector::Vector = new_vector!(start, end);
    let g = vec.to_tuple();
    assert_eq!(g.0, start);
    assert_eq!(g.1, end);
}

#[test]
fn split() {
    let start: coord::Coord = new_coord!(2.5, 3.3);
    let end: coord::Coord = new_coord!(2.5);

    let vec: vector::Vector = new_vector!(start, end);
    let (s, e, m) = vec.to_tuple();
    assert_eq!(s, start);
    assert_eq!(e, end);
    assert_eq!(m, vec.magnitude);
}

#[test]
fn operations() {
    let start: coord::Coord = new_coord!(1, 2);
    let end: coord::Coord = new_coord!(1, 2);
    let vec1: vector::Vector = new_vector!(start, end);

    let vec2: vector::Vector = new_vector!(start, end);

    assert_eq!(vec1 + vec2, new_vector!(new_coord!(2, 4), new_coord!(2, 4)));
    assert_eq!(vec1 - vec2, new_vector!(new_coord!(0, 0), new_coord!(0, 0)));
    assert_eq!(vec1 * vec2, new_vector!(new_coord!(1, 4), new_coord!(1, 4)));
    assert_eq!(vec1 / vec2, new_vector!(new_coord!(1, 1), new_coord!(1, 1)));
}

#[test]
#[should_panic(expected = "[Vector]: division by 0")]
fn division_zero() {
    let start: coord::Coord = new_coord!(1, 0);
    let end: coord::Coord = new_coord!(1, 0);
    let vec1: vector::Vector = new_vector!(start, end);

    let vec2: vector::Vector = new_vector!(start, end);

    assert_eq!(vec1 / vec2, new_vector!());
}
