use crate::define_request;

define_request!(
    "HimSearch",
    HimSearchRequest {
        onlyToday: bool,
        maxNum: isize,
    }
);
