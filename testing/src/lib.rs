
pub mod macros {
    #[macro_export]
    macro_rules! func {
        ($x:item) => {{
            println!("{}", $x)
        }};
    }
}
