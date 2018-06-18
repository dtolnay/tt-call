#![doc(html_root_url = "https://docs.rs/tt-call/1.0.0")]

#[macro_use]
mod predicate;
#[macro_use]
mod replace;
#[macro_use]
mod rust;
#[macro_use]
mod unexpected;

// In general it is not possible today in Rust to produce good error messages
// and good error spans at the same time. See:
//
//     https://github.com/rust-lang/rust/issues/44535
//
// Within this crate we prefer to produce errors with the right span, even if
// the message is not good. This scales much better to large input token
// streams.

/// Evaluate a tt-call macro and return its output to a given return
/// destination.
///
/// # Input
///
/// The input must start with an argument called `macro` which provides the name
/// of the macro for `tt_call!` to invoke.
///
///   - `macro = [{` name of macro to call `}]`
///
/// After that there may be any number of key-value pairs to be passed as
/// arguments to the macro being called.
///
///   - **`$(`**<br>
///     &emsp;&emsp;arbitrary key `= [{` arbitrary tokens `}]`<br>
///     **`)*`**
///
/// Finally a specification of the macro invocation to which this call should
/// return its output.
///
///   - `~~>` name of return destination macro `! {`<br>
///     &emsp;&emsp;arbitrary tokens<br>
///     `}`
///
/// # Examples
///
/// ```rust
/// #[macro_use]
/// extern crate tt_call;
///
/// macro_rules! print_is_ident {
///     {
///         token = [{ $token:tt }]
///         is_ident = [{ true }]
///     } => {
///         println!("turns out `{}` is an ident", stringify!($token));
///     };
///
///     {
///         token = [{ $token:tt }]
///         is_ident = [{ false }]
///     } => {
///         println!("nope, `{}` is not an ident", stringify!($token));
///     };
/// }
///
/// fn main() {
///     tt_call! {
///         macro = [{ tt_is_ident }]
///         input = [{ foo }]
///         ~~> print_is_ident! {
///             token = [{ foo }]
///         }
///     }
/// }
/// ```
///
/// If the invoked macro provides the entirety of the input to the return
/// destination macro, then the `!` and argument list may be omitted.
///
/// ```rust
/// #[macro_use]
/// extern crate tt_call;
///
/// macro_rules! print_is_ident {
///     {
///         is_ident = [{ true }]
///     } => {
///         println!("that token is an ident");
///     };
///
///     {
///         is_ident = [{ false }]
///     } => {
///         println!("nope, not an ident");
///     };
/// }
///
/// fn main() {
///     tt_call! {
///         macro = [{ tt_is_ident }]
///         input = [{ foo }]
///         ~~> print_is_ident
///     }
/// }
/// ```
///
/// And if the invoked macro produces exactly one output value and we just want
/// to expand to that output value, the destination macro may be omitted
/// entirely.
///
/// ```rust
/// #[macro_use]
/// extern crate tt_call;
///
/// fn main() {
///     let is_ident = tt_call! {
///         macro = [{ tt_is_ident }]
///         input = [{ foo }]
///     };
///     println!("{}", is_ident); // prints true or false
/// }
/// ```
#[macro_export]
macro_rules! tt_call {
    // Call macro and expand into the tokens of its one return value.
    {
        macro = [{ $m:ident }]
        $(
            $input:ident = [{ $($tokens:tt)* }]
        )*
    } => {
        $m! {
            (__tt_call_private tt_identity_return! {})
            $(
                $input = [{ $($tokens)* }]
            )*
        }
    };

    // Call macro and pass its return values to the given return destination.
    {
        macro = [{ $m:ident }]
        $(
            $input:ident = [{ $($tokens:tt)* }]
        )*
        ~~> $return:ident
    } => {
        $m! {
            (__tt_call_private $return ! {})
            $(
                $input = [{ $($tokens)* }]
            )*
        }
    };

    // Call macro and append its return values onto the invocation of the given
    // return destination without caller.
    {
        macro = [{ $m:ident }]
        $(
            $input:ident = [{ $($tokens:tt)* }]
        )*
        ~~> $return:ident ! {
            $(
                $name:ident = [{ $($state:tt)* }]
            )*
        }
    } => {
        $m! {
            (__tt_call_private $return! {
                $(
                    $name = [{ $($state)* }]
                )*
            })
            $(
                $input = [{ $($tokens)* }]
            )*
        }
    };

    // Call macro and append its return values onto the invocation of the given
    // return destination with caller.
    {
        macro = [{ $m:ident }]
        $(
            $input:ident = [{ $($tokens:tt)* }]
        )*
        ~~> $return:ident ! {
            $caller:tt
            $(
                $name:ident = [{ $($state:tt)* }]
            )*
        }
    } => {
        $m! {
            (__tt_call_private $return! {
                $caller
                $(
                    $name = [{ $($state)* }]
                )*
            })
            $(
                $input = [{ $($tokens)* }]
            )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tt_identity_return {
    // Macro returned one value.
    {
        $name:ident = [{ $($output:tt)* }]
    } => {
        $($output)*
    };

    // Macro parsed the entire input and returned one value.
    {
        $name:ident = [{ $($output:tt)* }]
        rest = [{ }]
    } => {
        $($output)*
    };

    // Unexpected: macro failed to parse the entire input.
    {
        $name:ident = [{ $($output:tt)* }]
        rest = [{ $($unexpected:tt)* }]
    } => {
        error_unexpected! {
            $($unexpected)*
        }
    };
}

/// Return zero or more output values to the caller macro.
///
/// # Input
///
/// The `tt_return!` invocation should be given a `$caller` to return to and a
/// sequence of zero or more named return values.
///
///   - **`$(`**<br>
///     &emsp;&emsp;arbitrary key `= [{` arbitrary tokens `}]`<br>
///     **`)*`**
///
/// # Example
///
/// ```rust
/// #[macro_use]
/// extern crate tt_call;
///
/// macro_rules! is_lowercase_self {
///     // Input token is `self`.
///     {
///         $caller:tt
///         input = [{ self }]
///     } => {
///         tt_return! {
///             $caller
///             is = [{ true }]
///         }
///     };
///
///     // Input token is anything other than `self`.
///     {
///         $caller:tt
///         input = [{ $other:tt }]
///     } => {
///         tt_return! {
///             $caller
///             is = [{ false }]
///         }
///     };
/// }
///
/// fn main() {
///     let is = tt_call! {
///         macro = [{ is_lowercase_self }]
///         input = [{ self }]
///     };
///     println!("{}", is);
/// }
/// ```
#[macro_export]
macro_rules! tt_return {
    {
        $caller:tt
        $(
            $output:ident = [{ $($tokens:tt)* }]
        )*
    } => {
        private_return! {
            $caller
            $(
                $output = [{ $($tokens)* }]
            )*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! private_return {
    {
        (__tt_call_private $caller:ident ! { $($state:tt)* })
        $($append:tt)*
    } => {
        $caller! {
            $($state)*
            $($append)*
        }
    };
}

/// Evaluate a condition and expand to one or the other of two branches.
///
/// # Input
///
///   - `condition = [{` name of predicate macro to invoke `}]`
///   - `input = [{` arbitrary tokens to pass as input to the predicate `}]`
///   - `true = [{` tokens to expand to if the predicate returns true `}]`
///   - `false = [{` and if the predicate returns false `}]`
///
/// The predicate macro must accept a single input value named `input`. It is
/// expected to return a single output value which may have any name but must
/// hold the tokens `true` or `false`. For example the built-in `tt_is_comma!`
/// predicate expands to `is_comma = [{ true }]` or `is_comma = [{ false }]`.
///
/// # Example
///
/// ```rust
/// #[macro_use]
/// extern crate tt_call;
///
/// macro_rules! parse_until_comma {
///     ($($input:tt)*) => {
///         tt_call! {
///             macro = [{ parse_until_comma_helper }]
///             before_comma = [{ }]
///             tokens = [{ $($input)* }]
///         }
///     };
/// }
///
/// macro_rules! parse_until_comma_helper {
///     {
///         $caller:tt
///         before_comma = [{ $($before:tt)* }]
///         tokens = [{ $first:tt $($rest:tt)* }]
///     } => {
///         tt_if! {
///             condition = [{ tt_is_comma }]
///             input = [{ $first }]
///             true = [{
///                 tt_return! {
///                     $caller
///                     before_comma = [{ $($before)* }]
///                 }
///             }]
///             false = [{
///                 parse_until_comma_helper! {
///                     $caller
///                     before_comma = [{ $($before)* $first }]
///                     tokens = [{ $($rest)* }]
///                 }
///             }]
///         }
///     };
/// }
///
/// fn main() {
///     assert_eq!(3, parse_until_comma!(1 + 2, three, four));
/// }
/// ```
#[macro_export]
macro_rules! tt_if {
    {
        condition = [{ $condition:ident }]
        input = [{ $($input:tt)* }]
        true = [{ $($then:tt)* }]
        false = [{ $($else:tt)* }]
    } => {
        tt_call! {
            macro = [{ $condition }]
            input = [{ $($input)* }]
            ~~> private_if_branch! {
                true = [{ $($then)* }]
                false = [{ $($else)* }]
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! private_if_branch {
    // Branch condition returned true.
    {
        true = [{ $($then:tt)* }]
        false = [{ $($else:tt)* }]
        $condition:ident = [{ true }]
    } => {
        $($then)*
    };

    // Branch condition returned false.
    {
        true = [{ $($then:tt)* }]
        false = [{ $($else:tt)* }]
        $condition:ident = [{ false }]
    } => {
        $($else)*
    };
}

/// Print arbitrary output values returned by a tt-call macro. This is valuable
/// for debugging.
/// <sup>**[tt-call]**</sup>
///
/// # Example
///
/// ```rust
/// #[macro_use]
/// extern crate tt_call;
///
/// fn main() {
///     tt_call! {
///         macro = [{ parse_type }]
///         input = [{ Vec<u8>, compressed=false }]
///         ~~> tt_debug
///     }
/// }
/// ```
///
/// The output is:
///
/// ```text
/// type = [{ Vec < u8 > }]
/// rest = [{ , compressed = false }]
/// ```
#[macro_export]
macro_rules! tt_debug {
    {
        $(
            $output:ident = [{ $($tokens:tt)* }]
        )*
    } => {
        $(
            println!(
                concat!(
                    stringify!($output),
                    " = [{{ ",
                    stringify!($($tokens)*),
                    " }}]",
                )
            );
        )*
    }
}
