// Copyright (c) 2024 Mariusz Zacirka
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// The macro generates test cases by assigning all given values to each variable. A test is generated for each permutation of given value sets.
/// Each value must be represented by an identifier, because for each variable and value name a module is created.
/// As a result you need to define a costant for each value.
/// For each call to the macro you need to specify:
/// * a `$title` of the test,
/// * at least one variable with a named constant assigned or a set of constants delimited by *or* keyword,
/// * a `$result` identifier which will get the value as a result of running the `$when_block` and will be available in the `$then_block`,
/// * the `$when_block` which should run the tested code and return its result,
/// * the `$then_block` which should check the result by running an assertion.
/// 
/// Example:
/// ```
/// fn some_code(a:i32, b:i32) -> i32 { (a + (b << 1)) << 1  }
/// mod tests {
///     use combination_tests::*;
///     const A1:i32 = 1;
///     const A2:i32 = 2;
///     const A3:i32 = 3;
///     const B10:i32 = 10;
///     const B20:i32 = 20;
/// 
///     test_permutations! {
///         title doubles_example;
///         let a = A1 or A2 or A3;
///         let b = B10 or B20;
///         when result = {
///             some_code(a, b)
///         }
///         then {
///             assert_eq!(result, 2*a + 4*b);
///         }
///     }
/// }
/// ```
/// It will generate 6 tests similiar to the following:
/// ```
/// mod tests {
///     mod doubles_example { mod a { mod A1 { mod b { mod B10 {
///         #[test]
///         fn test() {
///             let a = A1;
///             let b = B10;
///             let result = {
///                 some_code(a, b)
///             };
///             {
///                 assert_eq!(result, 2*a + 4*b);
///             }
///         }
///     }}}}}
/// }
/// ```
/// So the output will look like:
/// ```
/// // test tests::doubles_example::a::A1::b::B10::test ... ok
/// // test tests::doubles_example::a::A1::b::B20::test ... ok
/// // test tests::doubles_example::a::A2::b::B10::test ... ok
/// // test tests::doubles_example::a::A2::b::B20::test ... ok
/// // test tests::doubles_example::a::A3::b::B10::test ... ok
/// // test tests::doubles_example::a::A3::b::B20::test ... ok
/// ```
#[macro_export]
macro_rules! test_permutations {
    {
        title $title:ident;
        $(let $variable:ident = $($value:ident)or+;)+
        when $result:ident = $when_block:block
        then $then_block:block
    } => {
        mod $title {
            #[allow(unused_imports)]
            use super::*;
            test_permutations_impl! {
                single_lets {}
                multi_lets { $(let $variable = $($value)or+;)+ }
                when $result = $when_block
                then $then_block
            }
        }
    };
}

#[macro_export]
macro_rules! test_permutations_impl {
    {
        single_lets { $(let $single_var:ident = $single_val:ident;)* }
        multi_lets {}
        when $result:ident = $when_block:block
        then $then_block:block
    } => {
        #[test]
        fn test() {
            $(let $single_var = $single_val;)*
            let $result = $when_block;
            $then_block
        }
    };
    {
        single_lets { $(let $single_var:ident = $single_val:ident;)* } { let $single_var_extra:ident = $single_val_extra:ident; }
        multi_lets { $(let $multi_var_tail:ident = $($multi_val_tail:ident)or+;)* }
        when $result:ident = $when_block:block
        then $then_block:block
    } => {
        test_permutations_impl! {
            single_lets { $(let $single_var = $single_val;)* let $single_var_extra = $single_val_extra; }
            multi_lets { $(let $multi_var_tail = $($multi_val_tail)or+;)* }
            when $result = $when_block
            then $then_block
        }
    };
    {
        single_lets $single_lets:tt
        multi_lets { let $multi_var:ident = $($multi_val:ident)or+; }
        when $result:ident = $when_block:block
        then $then_block:block
    } => {
        #[allow(non_snake_case)]
        mod $multi_var {
            #[allow(unused_imports)]
            use super::*;
            $(
                #[allow(non_snake_case)]
                mod $multi_val {
                    #[allow(unused_imports)]
                    use super::*;
                    test_permutations_impl! {
                        single_lets $single_lets { let $multi_var = $multi_val; }
                        multi_lets {}
                        when $result = $when_block
                        then $then_block
                    }
                }
            )*
        }
    };
    {
        single_lets $single_lets:tt
        multi_lets { let $multi_var:ident = $($multi_val:ident)or+; $(let $multi_var_tail:ident = $($multi_val_tail:ident)or+;)+ }
        when $result:ident = $when_block:block
        then $then_block:block
    } => {
        test_permutations_impl! {
            single_lets $single_lets
            multi_lets { let $multi_var = $($multi_val)or+; } { $(let $multi_var_tail = $($multi_val_tail)or+;)+ }
            when $result = $when_block
            then $then_block
        }
    };
    {
        single_lets {}
        multi_lets { let $multi_var:ident = $($multi_val:ident)or+; } $multi_var_tail:tt
        when $result:ident = $when_block:block
        then $then_block:block
    } => {
        #[allow(non_snake_case)]
        mod $multi_var {
            #[allow(unused_imports)]
            use super::*;
            $(
                #[allow(non_snake_case)]
                mod $multi_val {
                    #[allow(unused_imports)]
                    use super::*;
                    test_permutations_impl! {
                        single_lets { let $multi_var = $multi_val; }
                        multi_lets $multi_var_tail
                        when $result = $when_block
                        then $then_block
                    }
                }
            )*
        }
    };
    {
        single_lets $single_lets:tt
        multi_lets { let $multi_var:ident = $($multi_val:ident)or+; } $multi_var_tail:tt
        when $result:ident = $when_block:block
        then $then_block:block
    } => {
        #[allow(non_snake_case)]
        mod $multi_var {
            #[allow(unused_imports)]
            use super::*;
            $(
                #[allow(non_snake_case)]
                mod $multi_val {
                    #[allow(unused_imports)]
                    use super::*;
                    test_permutations_impl! {
                        single_lets $single_lets { let $multi_var = $multi_val; }
                        multi_lets $multi_var_tail
                        when $result = $when_block
                        then $then_block
                    }
                }
            )*
        }
    };
}

#[cfg(test)]
mod tests {
    const A1:i32 = 1;
    const A2:i32 = 2;
    const A3:i32 = 3;

    const B10:i32 = 10;
    const B20:i32 = 20;
    const B30:i32 = 30;

    const C100:i32 = 100;
    const C200:i32 = 200;
    const C300:i32 = 300;

    test_permutations! {
        title expands_for_1_variable_with_1_value;
        let a = A1;
        when result = {
            a + 1
        }
        then {
            assert_eq!(result, 2);
        }
    }

    test_permutations! {
        title expands_for_1_variable_with_2_values;
        let a = A1 or A2;
        when result = {
            a + 1
        }
        then {
            assert_eq!(result, a + 1);
        }
    }

    test_permutations! {
        title expands_for_2_variables_first_with_1_value_and_second_with_2_values;
        let a = A1;
        let b = B10 or B20;
        when result = {
            2*a +  2*b
        }
        then {
            assert_eq!(result, 2*(a + b));
        }
    }

    test_permutations! {
        title expands_for_2_variables_first_with_2_values_and_second_with_1_value;
        let a = A1 or A2;
        let b = B10;
        when result = {
            2*a +  2*b
        }
        then {
            assert_eq!(result, 2*(a + b));
        }
    }

    test_permutations! {
        title expands_for_2_variables_each_with_2_values;
        let a = A1 or A2;
        let b = B10 or B20;
        when result = {
            2*a +  2*b
        }
        then {
            assert_eq!(result, 2*(a + b));
        }
    }

    test_permutations! {
        title expands_for_3_variables_each_with_3_values;
        let a = A1 or A2 or A3;
        let b = B10 or B20 or B30;
        let c = C100 or C200 or C300;
        when result = {
            2*a +  2*b + 2*c
        }
        then {
            assert_eq!(result, 2*(a + b + c));
        }
    }
}
