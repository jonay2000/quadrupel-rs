use cortex_m_semihosting::debug::{exit, EXIT_SUCCESS};

pub fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }

    exit(EXIT_SUCCESS);
}
