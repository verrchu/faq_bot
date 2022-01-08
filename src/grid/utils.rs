use std::fmt::Display;

pub(super) fn format_segment<S>(name: impl AsRef<str>, icon: Option<S>) -> String
where
    S: AsRef<str> + Display,
{
    icon.map(|icon| format!("{} {}", icon, name.as_ref()))
        .unwrap_or_else(|| name.as_ref().to_string())
}
