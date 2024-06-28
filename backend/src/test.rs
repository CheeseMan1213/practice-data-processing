use super::*;

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(QuadraticFormula::new(1.0, -3.0, 2.0).calculate(), (2.0, 1.0));
}
#[test]
fn test_quadratic_formula() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(QuadraticFormula::new(1.0, -3.0, 2.0).calculate(), (2.0, 1.0));
}
