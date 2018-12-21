#![allow(proc_macro_derive_resolution_fallback)] // Silence Diesel warnings that will be fixed in Diesel 1.4

table! {
    links (id) {
        id -> Int4,
        src -> Varchar,
        dest -> Varchar,
        hits -> Nullable<Int4>,
    }
}
