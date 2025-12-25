pub mod errors;

/// Check a payload for missing or empty string fields.
///
/// Returns a `Vec<&'static str>` with the names of fields that are missing or
/// contain an empty `String`. The macro checks `payload.field.as_ref()` and
/// attempts to downcast to `String`, so it is most useful with `Option<String>`
/// fields.
///
/// # Example
///
/// ```rust
/// struct Payload {
///     name: Option<String>,
///     email: Option<String>,
/// }
///
/// let payload = Payload { name: Some("".to_string()), email: None };
/// let missing = morde_rs::check_empty_fields!(payload, [name, email]);
/// assert_eq!(missing, vec!["name", "email"]);
/// ```
#[macro_export]
macro_rules! check_empty_fields {
    ($payload:expr, [$($field:ident),*]) => {
        {
            let mut missing = Vec::new();
            $(
                if $payload.$field.as_ref().map(|v| {
                    (v as &dyn ::std::any::Any)
                        .downcast_ref::<String>()
                        .map(|s| s.is_empty())
                        .unwrap_or(false)
                }).unwrap_or(true) {
                    missing.push(stringify!($field));
                }
            )*
            missing
        }
    };
}
