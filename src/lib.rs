/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann
 * License: MIT or Apache License 2.0
 **********************************************************************************************************************/
#![no_std]

// required attributes for the custom test framework
#![cfg_attr(test, no_main)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(ruspiro_test_framework::test_runner)]

// required reference to the test framework
#[cfg(test)]
extern crate ruspiro_test_framework;

#[cfg(test)]
#[no_mangle]
extern "C" fn run_test_main() {
    test_main();
}

#[cfg(test)]
mod tests {
    use ruspiro_test_framework::*;

    #[ruspiro_test]
    fn simple_unit_test_working() {
        assert_eq!(1+1, 2);
    }

    #[ruspiro_test]
    fn simple_unit_test_failing() {
        assert_eq!(1+1, 4);
    }

}
