#[macro_export]
macro_rules! conditional {
    ($condition: expr, $result: expr) => {{
        if $condition {
            Some($result)
        } else {
            None
        }
    }};
}
