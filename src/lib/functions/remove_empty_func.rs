//! # remove_empty_func
//!
//! Takes an environment variable name and returns its value of if its defined and contains some text.
//!

#[cfg(test)]
#[path = "./remove_empty_func_test.rs"]
mod remove_empty_func_test;

use crate::environment;

pub(crate) fn invoke(function_args: &Vec<String>) -> Vec<String> {
    if function_args.len() != 1 {
        error!("remove_empty expects only 1 argument (environment variable name)");
        panic!("remove_empty expects only 1 argument (environment variable name)");
    }

    let env_key = function_args[0].clone();

    let value = environment::get_env(&env_key, "");

    if value.len() > 0 {
        vec![value]
    } else {
        vec![]
    }
}
