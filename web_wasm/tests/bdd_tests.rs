use rust_gkat_wasm::syntax::{PureBDDGkat, Gkat};

#[test]
fn test_bdd_basic_operations() {
    let mut gkat = PureBDDGkat::new();

    // Test constants
    let zero = gkat.mk_zero();
    let one = gkat.mk_one();

    assert!(gkat.is_false(&zero));
    assert!(!gkat.is_false(&one));

    // Test variables
    let a = gkat.mk_var("a".to_string());
    let b = gkat.mk_var("b".to_string());

    // Test NOT
    let not_a = gkat.mk_not(&a);
    assert!(!gkat.is_equiv(&a, &not_a));

    // Test AND
    let _a_and_b = gkat.mk_and(&a, &b);
    let a_and_zero = gkat.mk_and(&a, &zero);
    let a_and_one = gkat.mk_and(&a, &one);

    assert!(gkat.is_false(&a_and_zero));
    assert!(gkat.is_equiv(&a, &a_and_one));

    // Test OR
    let _a_or_b = gkat.mk_or(&a, &b);
    let a_or_zero = gkat.mk_or(&a, &zero);
    let a_or_one = gkat.mk_or(&a, &one);

    assert!(gkat.is_equiv(&a, &a_or_zero));
    assert!(gkat.is_equiv(&one, &a_or_one));

    // Test complex expressions
    let not_b = gkat.mk_not(&b);
    let expr1 = gkat.mk_and(&a, &not_b);
    let expr2 = gkat.mk_and(&not_b, &a);

    assert!(gkat.is_equiv(&expr1, &expr2));
}

#[test]
fn test_bdd_laws() {
    let mut gkat = PureBDDGkat::new();

    let a = gkat.mk_var("a".to_string());
    let b = gkat.mk_var("b".to_string());
    let c = gkat.mk_var("c".to_string());

    // Test associativity
    let b_and_c = gkat.mk_and(&b, &c);
    let a_and_b_and_c = gkat.mk_and(&a, &b_and_c);

    let a_and_b = gkat.mk_and(&a, &b);
    let a_and_b_and_c2 = gkat.mk_and(&a_and_b, &c);

    assert!(gkat.is_equiv(&a_and_b_and_c, &a_and_b_and_c2));

    let b_or_c = gkat.mk_or(&b, &c);
    let a_or_b_or_c = gkat.mk_or(&a, &b_or_c);

    let a_or_b = gkat.mk_or(&a, &b);
    let a_or_b_or_c2 = gkat.mk_or(&a_or_b, &c);

    assert!(gkat.is_equiv(&a_or_b_or_c, &a_or_b_or_c2));

    // Test commutativity
    let a_and_b = gkat.mk_and(&a, &b);
    let b_and_a = gkat.mk_and(&b, &a);
    assert!(gkat.is_equiv(&a_and_b, &b_and_a));

    let a_or_b = gkat.mk_or(&a, &b);
    let b_or_a = gkat.mk_or(&b, &a);
    assert!(gkat.is_equiv(&a_or_b, &b_or_a));

    // Test distributivity
    let b_or_c = gkat.mk_or(&b, &c);
    let dist1 = gkat.mk_and(&a, &b_or_c);

    let a_and_b = gkat.mk_and(&a, &b);
    let a_and_c = gkat.mk_and(&a, &c);
    let dist2 = gkat.mk_or(&a_and_b, &a_and_c);

    assert!(gkat.is_equiv(&dist1, &dist2));

    // Test De Morgan's laws
    let a_and_b = gkat.mk_and(&a, &b);
    let not_a_and_b = gkat.mk_not(&a_and_b);

    let not_a = gkat.mk_not(&a);
    let not_b = gkat.mk_not(&b);
    let not_a_or_not_b = gkat.mk_or(&not_a, &not_b);

    assert!(gkat.is_equiv(&not_a_and_b, &not_a_or_not_b));

    let a_or_b = gkat.mk_or(&a, &b);
    let not_a_or_b = gkat.mk_not(&a_or_b);

    let not_a = gkat.mk_not(&a);
    let not_b = gkat.mk_not(&b);
    let not_a_and_not_b = gkat.mk_and(&not_a, &not_b);

    assert!(gkat.is_equiv(&not_a_or_b, &not_a_and_not_b));
}