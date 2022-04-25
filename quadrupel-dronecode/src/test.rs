use cortex_m_semihosting::debug::{exit, EXIT_SUCCESS};
use cortex_m_semihosting::hprintln;

pub fn test_runner(tests: &[&dyn Fn()]) {
    hprintln!("--- RUNNING {} TESTS ---", tests.len());
    for test in tests {
        test();
    }

    hprintln!("--- ALL TESTS SUCCESSFUL ---");
    exit(EXIT_SUCCESS);
}

#[test_case]
fn test() {
    assert_eq!(3 + 3, 6);
}
