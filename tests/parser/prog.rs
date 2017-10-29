extern crate cmm;

use cmm::ast::*;

#[test]
fn empty() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#""#).unwrap();

    assert!(errors.is_empty());
    assert_eq!(format!("[]"), format!("{:?}", actual));
}

#[test]
fn proto_void() {
    let mut errors = Vec::new();
    let mut errors2 = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        void foo (void);
    "#).unwrap();

    let actual2 = cmm::parse(&mut errors2, r#"
        void foo ();
    "#).unwrap();

    let expected = CProto {
        ret: None,
        name: "foo",
        params: vec![],
    };

    assert!(errors.is_empty());
    assert!(errors2.is_empty());
    assert_eq!(format!("[{:?}]", expected.clone()), format!("{:?}", actual));
    assert_eq!(format!("[{:?}]", expected.clone()), format!("{:?}", actual2));
}

#[test]
fn proto_types() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int foo (int a, char b);
    "#).unwrap();

    let expected = CProto {
        ret: Some(CType::Int),
        name: "foo",
        params: vec![(CType::Int, "a"), (CType::Char, "b")],
    };

    assert!(errors.is_empty());
    assert_eq!(format!("[{:?}]", expected), format!("{:?}", actual));
}

#[test]
fn proto_names_good() {
    let mut errors = Vec::new();
    let mut errors2 = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int foo42 ();
    "#).unwrap();

    let actual2 = cmm::parse(&mut errors2, r#"
        int foo_42 ();
    "#).unwrap();

    let expected = CProto {
        ret: Some(CType::Int),
        name: "foo42",
        params: vec![],
    };

    let expected2 = CProto {
        ret: Some(CType::Int),
        name: "foo_42",
        params: vec![],
    };

    assert!(errors.is_empty());
    assert!(errors2.is_empty());
    assert_eq!(format!("[{:?}]", expected), format!("{:?}", actual));
    assert_eq!(format!("[{:?}]", expected2), format!("{:?}", actual2));
}

#[test]
fn proto_names_bad() {
    let mut errors = Vec::new();
    let mut errors2 = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int 42foo42 ();
    "#);

    let actual2 = cmm::parse(&mut errors2, r#"
        int _foo_42 ();
    "#);

    // assert_eq!(1, errors.len());
    // assert_eq!(1, errors2.len());
    assert!(actual.is_err());
    assert!(actual2.is_err());
}

#[test]
fn proto_mult() {
    let mut errors = Vec::new();

    let actual = cmm::parse(&mut errors, r#"
        int foo(int a), bar(char b);
    "#).unwrap();

    let expected = vec![CProto {
        ret: Some(CType::Int),
        name: "foo",
        params: vec![(CType::Int, "a")],
    }, CProto {
        ret: Some(CType::Int),
        name: "bar",
        params: vec![(CType::Char, "b")],
    }];

    assert!(errors.is_empty());
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}