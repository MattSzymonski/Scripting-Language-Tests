mod bloat_gen_no_export;
mod modules;
mod test;

pub fn beta() -> String {
    "beta".to_string()
}

pub fn alpha() {
    println!("alpha {}", beta());
    println!("alphawawdawdw");
}

#[unsafe(no_mangle)]
pub extern "C" fn update(tick: i32) {
    println!(
        "12aawedwwaswadddaawdwdwdawawdawdsddasdadwwasasdawdawsaawdwdawdawddaasdxdadwawd1awdadw2awdawd"
    );
    alpha();
    test::alpha();

    // Fan out into the full interconnected module graph (all 50 modules).
    let module_result = modules::run_update(tick);
    println!("        modules::run_update({tick}) = {module_result}");
}

#[unsafe(no_mangle)]
pub extern "C" fn compute(x: i32, y: i32) -> i32 {
    // Combine the original formula with the interconnected module graph.
    x * y * 3 + modules::run_compute(x, y)
}
