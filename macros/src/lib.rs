#[macro_export]

macro_rules! hashmap {
    ( $( $k: expr => $v: expr ),* $(,)? ) => {
        {
            let mut kvpair = std::collections::HashMap::new();
            $(
                kvpair.insert($k, $v);
            )*

            kvpair
        }
    };
}
