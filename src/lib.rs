use std::any::Any;
use std::collections::HashMap;
use lazy_regex::regex;

/// Creates a `HashMap<String, Box<dyn Any>>` from a list of key-value pairs.
///
/// ## Example
///
/// ```
/// use apply_string_args::string_args;
/// fn main() {
///     let map = string_args!{
///         "a" => "foo",
///         "b" => "bar",
///     };
///     assert_eq!(*map[&"a".to_owned()], "foo");
///     assert_eq!(*map[&"b".to_owned()], "bar");
///     assert_eq!(map.get(&"c".to_owned()), None);
/// }
/// ```
#[macro_export]
macro_rules! string_args {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(string_args!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { string_args!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let r_cap = string_args!(@count $($key),*);
            let mut r_map = HashMap::<String, Box<dyn Any>>::with_capacity(r_cap);
            $(
                let _ = r_map.insert($key.to_string(), Box::new($value));
            )*
            r_map
        }
    };
}

pub fn apply(base: impl AsRef<str>, vars: &HashMap<String, Box<dyn Any>>, convert: fn(arg: &Box<dyn Any>) -> String) -> String {
    regex!(r"\$(\$|[A-Za-z0-9]+)").replace_all(base.as_ref(), |s: &regex::Captures<'_>| {
        let s = s.get(0).unwrap().as_str();
        if s == "$$" {
            "$".to_owned()
        } else {
            let v = vars.get(&s.to_string().replace("$", ""));
            if let Some(v) = v { convert(v) } else { "None".to_owned() }
        }
    }).as_ref().to_string()
}