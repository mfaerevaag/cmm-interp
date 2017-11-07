extern crate cmm;

use cmm::engine;
use cmm::env::{FuncTab, SymTab, SymVal};

#[test]
fn bin_op() {
    let vtab = FuncTab::new();
    let global = SymTab::new();
    let local = SymTab::new();

    let ast = cmm::parse_expr(&mut vec![], r#"
1 + 1
"#).unwrap();

    let actual = engine::run_expr(&ast, &vtab, &global, &local);

    let expected = SymVal::Num(2);

    assert_eq!(expected, actual);
}

#[test]
fn rel_op() {
    let vtab = FuncTab::new();
    let global = SymTab::new();
    let local = SymTab::new();

    let ast = cmm::parse_expr(&mut vec![], r#"
1 == 1
"#).unwrap();

    let actual = engine::run_expr(&ast, &vtab, &global, &local);

    let expected = SymVal::Bool(true);

    assert_eq!(expected, actual);
}

#[test]
fn log_op() {
    let vtab = FuncTab::new();
    let global = SymTab::new();
    let local = SymTab::new();

    let ast = cmm::parse_expr(&mut vec![], r#"
(1 == 1) && (1 != 0)
"#).unwrap();

    let actual = engine::run_expr(&ast, &vtab, &global, &local);

    let expected = SymVal::Bool(true);

    assert_eq!(expected, actual);
}