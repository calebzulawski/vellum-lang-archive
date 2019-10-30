#[macro_export]
macro_rules! define {
    {
        $($input:tt)*
    } => {
        tt_call::tt_call! {
            macro  = [{ $crate::define_impl }]
            dollar = [{ $ }]
            input  = [{ $($input)* }]
            output = [{ }]
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! define_impl {
    // empty statement does nothing
    {
        $caller:tt
        dollar = [{ $ }]
        input  = [{ }]
        output = [{ $($tokens:tt)* }]
    } => {
        tt_call::tt_return! {
            $caller
            output = [{ $($tokens)* }]
        }
    };

    // impl statement
    {
        $caller:tt
        dollar = [{ $ds:tt }]
        input  = [{ impl $name:ident $($args:ident)* = { $($value:tt)* }; $($tail:tt)* }]
        output = [{ $($output:tt)* }]
    } => {
        tt_call::tt_return! {
            $caller
            output = [{
                tt_call::tt_call! {
                    macro  = [{ $crate::define_impl }]
                    dollar = [{ $ }]
                    input  = [{ $($tail)* }]
                    output = [{
                        $($output)*
                        macro_rules! $name {
                            {
                                $ds caller:tt
                                args = [{ $($ds $args:expr),* }]
                            } => {
                                tt_call::tt_return! {
                                    $ds caller
                                    output = [{ { $($value)* } }]
                                }
                            }
                        }
                    }]
                }
            }]
        }
    }
}
