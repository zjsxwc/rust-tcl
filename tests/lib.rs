extern crate tcl;

#[test]
fn create_interp() {
    tcl::Interpreter::new();
}
#[test]
fn interp_safety() {
    let mut interp = tcl::Interpreter::new();
    assert_eq!(interp.is_safe(), false);
    assert_eq!(interp.make_safe(), true);
    assert_eq!(interp.is_safe(), true);
}