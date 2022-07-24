#[macro_export]
macro_rules! hashmap {
    (<$key_ty:ty, $value_ty:ty> [$($key:expr => $value:expr),*]) => {{
        let mut map: HashMap<$key_ty, $value_ty> = HashMap::new();
        $(map.insert($key.to_owned(), $value);)*

        map
    }};
}
