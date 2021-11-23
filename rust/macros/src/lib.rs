#[macro_export]
macro_rules! hashmap {
    // With trailing comma.
    (
        $( $k:expr => $v:expr,)*
    ) =>
    {
        {
            use ::std::collections::HashMap;

            let mut hm = HashMap::new();
            $(
                *hm.entry($k).or_insert($v) = $v;
            )*

            hm
        }
    };

    // Without trailing comma.
    (
        $( $k:expr => $v:expr),*
    ) =>
    {
        {
            use ::std::collections::HashMap;

            let mut hm = HashMap::new();
            $(
                *hm.entry($k).or_insert($v) = $v;
            )*

            hm
        }
    };
}
