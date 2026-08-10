mod bloat_gen_no_export;
mod test;

pub fn beta() -> String {
    "beta".to_string()
}

pub fn alpha() {
    println!("alpha {}", beta());
    println!("alphaww");
}

#[unsafe(no_mangle)]
pub extern "C" fn update(tick: i32) {
    println!("12aawedwwaswadddwdawawdsdsadxd");
    alpha();
    test::alpha();
}

#[unsafe(no_mangle)]
pub extern "C" fn compute(x: i32, y: i32) -> i32 {
    x * y * 3
}
