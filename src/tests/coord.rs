use crate::*;

#[test]
fn new_coord() {
    let point: coord::Coord = new_coord!(1, 2);
    assert_eq!(point.x, 1.0);
    assert_eq!(point.y, 2.0);

    let point: coord::Coord = new_coord!(1.0, 2.0);
    assert_eq!(point.x, 1.0);
    assert_eq!(point.y, 2.0);

    let point: coord::Coord = new_coord!();
    assert_eq!(point.x, 0.0);
    assert_eq!(point.y, 0.0);

    let point: coord::Coord = new_coord!(29);
    assert_eq!(point.x, 0.0);
    assert_eq!(point.y, 29.0);
}

#[test]
fn to_vec() {
    let a: coord::Coord = new_coord!(1, 2.0);

    // to vec
    let g: Vec<f64> = a.to_vec();

    assert_eq!(g[0], a.x);
    assert_eq!(g[1], a.y);
}

#[test]
fn to_tuple() {
    let a: coord::Coord = new_coord!(135, 23.0);

    // to tuple
    let i: (f64, f64) = a.to_tuple();

    assert_eq!(i.0, a.x);
    assert_eq!(i.1, a.y);
}

#[test]
fn split() {
    let a: coord::Coord = new_coord!(135, 23.0);

    // split
    let (r, l) = a.to_tuple();

    assert_eq!(r, a.x);
    assert_eq!(l, a.y);
}

#[test]
fn operations() {
    let a: coord::Coord = new_coord!(1.0, 2.0);
    let b: coord::Coord = new_coord!(1.0, 2.0);
    let c: coord::Coord = new_coord!(1.0, 2.9);

    assert_eq!(a == b, true);
    assert_eq!(a == c, false);

    assert_eq!(a + b, new_coord!(2, 4));
    assert_eq!(a - b, new_coord!(0, 0));
    assert_eq!(a * b, new_coord!(1, 4));
    assert_eq!(a / b, new_coord!(1, 1));
}

#[test]
#[should_panic(expected = "[Coord]: division by 0")]
fn division_zero() {
    let a: coord::Coord = new_coord!(1, 2);
    let b: coord::Coord = new_coord!(1, 0);

    assert_eq!(a / b, new_coord!());
}
