macro_rules! plugins {
    ( $( $x:ident ),* ) => {
        $(extern crate $x;)*

        pub fn init() -> Vec<Box<AgentPlugin>> {
            use std::panic;
            let mut v: Vec<Box<AgentPlugin>> = vec!();
            let p = panic::take_hook();
            panic::set_hook(Box::new(|_info| {
                // do nothing
            }));
            $(
                match std::panic::catch_unwind(|| $x::new()) {
                    Ok(x) => {v.push(Box::new(x))},
                    Err(_) => { }
                };
            )*
            panic::set_hook(p);
            v
        }
    }
}
