#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! private_parse_path {
    // Entry point. Dup tokens.
    {
        $caller:tt
        input = [{ $($input:tt)* }]
    } => {
        private_parse_path! {
            $caller
            tokens = [{ $($input)* }]
            _tokens = [{ $($input)* }]
        }
    };

    // Parse absolute path.
    {
        $caller:tt
        tokens = [{ :: $segment:ident $($rest:tt)* }]
        _tokens = [{ $colons:tt $($dup:tt)* }]
    } => {
        tt_call! {
            macro = [{ private_parse_possibly_empty_path_after_ident }]
            input = [{ $($rest)* }]
            ~~> private_parse_path! {
                $caller
                prefix = [{ $colons $segment }]
            }
        }
    };

    // Parse relative path.
    {
        $caller:tt
        tokens = [{ $segment:ident $($rest:tt)* }]
        _tokens = [{ $($dup:tt)* }]
    } => {
        tt_call! {
            macro = [{ private_parse_possibly_empty_path_after_ident }]
            input = [{ $($rest)* }]
            ~~> private_parse_path! {
                $caller
                prefix = [{ $segment }]
            }
        }
    };

    // Return path.
    {
        $caller:tt
        prefix = [{ $($prefix:tt)* }]
        path = [{ $($path:tt)* }]
        rest = [{ $($rest:tt)* }]
    } => {
        tt_return! {
            $caller
            path = [{ $($prefix)* $($path)* }]
            rest = [{ $($rest)* }]
        }
    };

}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! private_parse_possibly_empty_path_after_ident {
    // Entry point. Dup tokens.
    {
        $caller:tt
        input = [{ $($input:tt)* }]
    } => {
        private_parse_possibly_empty_path_after_ident! {
            $caller
            path = [{ }]
            tokens = [{ $($input)* }]
            _tokens = [{ $($input)* }]
        }
    };

    // Parse empty angle brackets.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ < > $($rest:tt)* }]
        _tokens = [{ $lt:tt $gt:tt $($dup:tt)* }]
    } => {
        private_parse_possibly_empty_path_after_close_angle! {
            $caller
            path = [{ $($path)* $lt $gt }]
            tokens = [{ $($rest)* }]
        }
    };

    // Unexpected: input ends after open angle bracket.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ < }]
        _tokens = [{ $unexpected:tt }]
    } => {
        error_unexpected! {
            $unexpected
        }
    };

    // Parse generic param inside of angle brackets.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ < $($rest:tt)+ }]
        _tokens = [{ $lt:tt $($dup:tt)* }]
    } => {
        tt_call! {
            macro = [{ private_parse_generic_param }]
            input = [{ $($rest)* }]
            ~~> private_parse_in_angle_brackets! {
                $caller
                prefix = [{ $($path)* $lt }]
            }
        }
    };

    // Parse empty turbofish.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ :: < > $($rest:tt)* }]
        _tokens = [{ $colons:tt $lt:tt $gt:tt $($dup:tt)* }]
    } => {
        private_parse_possibly_empty_path_after_close_angle! {
            $caller
            path = [{ $($path)* $colons $lt $gt }]
            tokens = [{ $($rest)* }]
        }
    };

    // Parse generic param inside of turbofish.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ :: < $($rest:tt)+ }]
        _tokens = [{ $colons:tt $lt:tt $($dup:tt)* }]
    } => {
        tt_call! {
            macro = [{ private_parse_generic_param }]
            input = [{ $($rest)* }]
            ~~> private_parse_in_angle_brackets! {
                $caller
                prefix = [{ $($path)* $colons $lt }]
            }
        }
    };

    // Parse parenthesized parameter data.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ ($($args:tt)*) $($rest:tt)* }]
        _tokens = [{ $original:tt $($dup:tt)* }]
    } => {
        tt_call! {
            macro = [{ private_validate_fn_path_args }]
            tokens = [{ $($args)* }]
            ~~> private_parse_path_after_fn_args! {
                $caller
                path = [{ $($path)* $original }]
                tokens = [{ $($rest)* }]
            }
        }
    };

    // Anything after close angle is allowed after ident.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ $($tokens:tt)* }]
        _tokens = [{ $($dup:tt)* }]
    } => {
        private_parse_possibly_empty_path_after_close_angle! {
            $caller
            path = [{ $($path)* }]
            tokens = [{ $($tokens)* }]
        }
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! private_parse_possibly_empty_path_after_close_angle {
    // Entry point. Dup tokens.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ $($tokens:tt)* }]
    } => {
        private_parse_possibly_empty_path_after_close_angle! {
            $caller
            path = [{ $($path)* }]
            tokens = [{ $($tokens)* }]
            _tokens = [{ $($tokens)* }]
        }
    };

    // Parse path segment.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ :: $segment:ident $($rest:tt)* }]
        _tokens = [{ $colons:tt $($dup:tt)* }]
    } => {
        private_parse_possibly_empty_path_after_ident! {
            $caller
            path = [{ $($path)* $colons $segment }]
            tokens = [{ $($rest)* }]
            _tokens = [{ $($rest)* }]
        }
    };

    // Unexpected: double colon is followed by something other than ident.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ :: $($unexpected:tt)+ }]
        _tokens = [{ $($dup:tt)* }]
    } => {
        error_unexpected! {
            $($unexpected)*
        }
    };

    // Not a double colon. End of path.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ $($rest:tt)* }]
        _tokens = [{ $($dup:tt)* }]
    } => {
        tt_return! {
            $caller
            path = [{ $($path)* }]
            rest = [{ $($rest)* }]
        }
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! private_parse_in_angle_brackets {
    // Entry point. Dup rest tokens.
    {
        $caller:tt
        prefix = [{ $($path:tt)* }]
        param = [{ $($param:tt)* }]
        rest = [{ $($rest:tt)* }]
    } => {
        private_parse_in_angle_brackets! {
            $caller
            prefix = [{ $($path)* }]
            param = [{ $($param)* }]
            rest = [{ $($rest)* }]
            _rest = [{ $($rest)* }]
        }
    };

    // End of angle bracketed path parameters. Parse rest of path.
    {
        $caller:tt
        prefix = [{ $($path:tt)* }]
        param = [{ $($param:tt)* }]
        rest = [{ > $($rest:tt)* }]
        _rest = [{ $gt:tt $($dup:tt)* }]
    } => {
        private_parse_possibly_empty_path_after_close_angle! {
            $caller
            path = [{ $($path)* $($param)* $gt }]
            tokens = [{ $($rest)* }]
        }
    };

    // Split a `>>` token into `> >`.
    {
        $caller:tt
        prefix = [{ $($path:tt)* }]
        param = [{ $($param:tt)* }]
        rest = [{ >> $($rest:tt)* }]
        _rest = [{ $($dup:tt)* }]
    } => {
        tt_return! {
            $caller
            path = [{ $($path)* $($param)* > }]
            rest = [{ > $($rest)* }]
        }
    };

    // End of angle bracketed path paremeters with trailing comma.
    {
        $caller:tt
        prefix = [{ $($path:tt)* }]
        param = [{ $($param:tt)* }]
        rest = [{ , > $($rest:tt)* }]
        _rest = [{ $comma:tt $gt:tt $($dup:tt)* }]
    } => {
        private_parse_possibly_empty_path_after_close_angle! {
            $caller
            path = [{ $($path)* $($param)* $comma $gt }]
            tokens = [{ $($rest)* }]
        }
    };

    // Split a `>>` token into `> >`.
    {
        $caller:tt
        prefix = [{ $($path:tt)* }]
        param = [{ $($param:tt)* }]
        rest = [{ , >> $($rest:tt)* }]
        _rest = [{ $comma:tt $($dup:tt)* }]
    } => {
        tt_return! {
            $caller
            path = [{ $($path)* $($param)* $comma > }]
            rest = [{ > $($rest)* }]
        }
    };

    // Parse generic parameter after comma.
    {
        $caller:tt
        prefix = [{ $($path:tt)* }]
        param = [{ $($param:tt)* }]
        rest = [{ , $($rest:tt)+ }]
        _rest = [{ $comma:tt $($dup:tt)* }]
    } => {
        tt_call! {
            macro = [{ private_parse_generic_param }]
            input = [{ $($rest)* }]
            ~~> private_parse_in_angle_brackets! {
                $caller
                prefix = [{ $($path)* $($param)* $comma }]
            }
        }
    };

    // Unexpected: generic parameter is not followed by `>` or comma.
    {
        $caller:tt
        prefix = [{ $($path:tt)* }]
        param = [{ $($param:tt)* }]
        rest = [{ $($unexpected:tt)+ }]
        _rest = [{ $($dup:tt)* }]
    } => {
        error_unexpected! {
            $($unexpected)*
        }
    };

    // Unexpected: input ends inside of angle brackets.
    {
        $caller:tt
        prefix = [{ $($path:tt)* }]
        param = [{ $($param:tt)+ }]
        rest = [{ }]
        _rest = [{ }]
    } => {
        error_unexpected_last! {
            $($param)*
        }
    };

}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! private_parse_generic_param {
    // Parse lifetime parameters.
    {
        $caller:tt
        input = [{ $lifetime:lifetime $($rest:tt)* }]
    } => {
        tt_return! {
            $caller
            param = [{ $lifetime }]
            rest = [{ $($rest)* }]
        }
    };

    // Parse associated type parameter.
    {
        $caller:tt
        input = [{ $assoc:ident = $($rest:tt)+ }]
    } => {
        tt_call! {
            macro = [{ parse_type }]
            input = [{ $($rest)* }]
            ~~> private_parse_generic_param! {
                $caller
                assoc = [{ $assoc = }]
            }
        }
    };

    // Return from parsing associated type parameter.
    {
        $caller:tt
        assoc = [{ $assoc:ident $eq:tt }]
        type = [{ $($ty:tt)* }]
        rest = [{ $($rest:tt)* }]
    } => {
        tt_return! {
            $caller
            param = [{ $assoc $eq $($ty)* }]
            rest = [{ $($rest)* }]
        }
    };

    // Parse type parameter.
    {
        $caller:tt
        input = [{ $($input:tt)+ }]
    } => {
        tt_call! {
            macro = [{ private_parse_type_with_plus }]
            input = [{ $($input)* }]
            ~~> private_parse_generic_param! {
                $caller
            }
        }
    };

    // Return from parsing type parameter.
    {
        $caller:tt
        type = [{ $($ty:tt)* }]
        rest = [{ $($rest:tt)* }]
    } => {
        tt_return! {
            $caller
            param = [{ $($ty)* }]
            rest = [{ $($rest)* }]
        }
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! private_validate_fn_path_args {
    // Function arguments are empty.
    {
        $caller:tt
        tokens = [{ }]
    } => {
        tt_return! {
            $caller
        }
    };

    // Validate first function argument type.
    {
        $caller:tt
        tokens = [{ $($rest:tt)+ }]
    } => {
        tt_call! {
            macro = [{ parse_type }]
            input = [{ $($rest)* }]
            ~~> private_validate_fn_path_args! {
                $caller
            }
        }
    };

    // All function argument types are valid.
    {
        $caller:tt
        type = [{ $($ty:tt)* }]
        rest = [{ }]
    } => {
        tt_return! {
            $caller
        }
    };

    // Validate next function argument type after comma.
    {
        $caller:tt
        type = [{ $($ty:tt)* }]
        rest = [{ , $($rest:tt)* }]
    } => {
        private_validate_fn_path_args! {
            $caller
            tokens = [{ $($rest)* }]
        }
    };

    // Unexpected: function argument type is not followed by comma.
    {
        $caller:tt
        type = [{ $($ty:tt)* }]
        rest = [{ $($unexpected:tt)+ }]
    } => {
        error_unexpected! {
            $($unexpected)*
        }
    };
}

#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! private_parse_path_after_fn_args {
    // Entry point. Dup tokens.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ $($tokens:tt)* }]
    } => {
        private_parse_path_after_fn_args! {
            $caller
            path = [{ $($path)* }]
            tokens = [{ $($tokens)* }]
            _tokens = [{ $($tokens)* }]
        }
    };

    // Parse function return type.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ -> $($rest:tt)* }]
        _tokens = [{ $arrow:tt $($dup:tt)* }]
    } => {
        tt_call! {
            macro = [{ parse_type }]
            input = [{ $($rest)* }]
            ~~> private_parse_path_after_fn_args! {
                $caller
                path = [{ $($path)* $arrow }]
            }
        }
    };

    // Function has default return type.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        tokens = [{ $($rest:tt)* }]
        _tokens = [{ $($dup:tt)* }]
    } => {
        tt_return! {
            $caller
            path = [{ $($path)* }]
            rest = [{ $($rest)* }]
        }
    };

    // Return from parsing function return type.
    {
        $caller:tt
        path = [{ $($path:tt)* }]
        type = [{ $($ret:tt)* }]
        rest = [{ $($rest:tt)* }]
    } => {
        tt_return! {
            $caller
            path = [{ $($path)* $($ret)* }]
            rest = [{ $($rest)* }]
        }
    };
}
