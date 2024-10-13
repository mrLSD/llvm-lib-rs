/// Get a mutable pointer for the collection.
#[macro_export]
macro_rules! to_mut_ptr {
    ($collection:expr) => {{
        use $crate::GetRef;
        let mut vec = $collection.iter().map(|v| v.get_ref()).collect::<Vec<_>>();
        if vec.is_empty() {
            std::ptr::null_mut()
        } else {
            vec.as_mut_ptr()
        }
    }};
}

/// Get a mutable pointer for the collection with Map function.
#[macro_export]
macro_rules! map_mut_ptr {
    ($collection:expr, $map_fn:expr) => {{
        let mut vec = $collection.iter().map($map_fn).collect::<Vec<_>>();
        if vec.is_empty() {
            std::ptr::null_mut()
        } else {
            vec.as_mut_ptr()
        }
    }};
}
