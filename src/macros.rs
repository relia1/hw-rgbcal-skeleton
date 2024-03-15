/// Macros

#[macro_export]
macro_rules! async_getter {
    (
        $(#[$attr:meta])*
        $fn_name:ident, $var_name:ident, $global_mutex:ident, $return:ty
    ) => {
        async fn $fn_name() -> $return {
            let $var_name = $global_mutex.lock().await;
            *$var_name
        }
    };
}

#[macro_export]
macro_rules! async_setter {
    (
        $(#[$attr:meta])*
        $fn_name:ident, $var_name:ident, $type:ty, $global_mutex:ident, $block:expr
    ) => {
        async fn $fn_name<F>(setter: F)
        where
            F: FnOnce(&mut $type),
        {
            let mut $var_name = $global_mutex.lock().await;
            setter(&mut $var_name);
        }
    };
}
