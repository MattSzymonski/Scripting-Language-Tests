// test — supplementary module for the game DLL.
// Called by update() via test::alpha().

pub fn beta() -> String {
    "beta from test moduleXXXX".to_string()
}

pub fn alpha() {
    println!("  test::aawdaWDlpha says: {}", beta());
}
