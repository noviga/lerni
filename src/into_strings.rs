pub(crate) trait IntoStrings {
    fn into_strings(self) -> Vec<String>;
}

impl IntoStrings for &str {
    fn into_strings(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

// TODO: Rewrite using macros
impl IntoStrings for (&str, &str) {
    fn into_strings(self) -> Vec<String> {
        vec![self.0.to_string(), self.1.to_string()]
    }
}

impl IntoStrings for (&str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        vec![self.0.to_string(), self.1.to_string(), self.2.to_string()]
    }
}

impl IntoStrings for (&str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
        ]
    }
}

impl IntoStrings for (&str, &str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
            self.4.to_string(),
        ]
    }
}

impl IntoStrings for (&str, &str, &str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
            self.4.to_string(),
            self.5.to_string(),
        ]
    }
}

impl IntoStrings for (&str, &str, &str, &str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
            self.4.to_string(),
            self.5.to_string(),
            self.6.to_string(),
        ]
    }
}

impl IntoStrings for (&str, &str, &str, &str, &str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
            self.4.to_string(),
            self.5.to_string(),
            self.6.to_string(),
            self.7.to_string(),
        ]
    }
}

impl IntoStrings for (&str, &str, &str, &str, &str, &str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
            self.4.to_string(),
            self.5.to_string(),
            self.6.to_string(),
            self.7.to_string(),
            self.8.to_string(),
        ]
    }
}

impl IntoStrings for (&str, &str, &str, &str, &str, &str, &str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
            self.4.to_string(),
            self.5.to_string(),
            self.6.to_string(),
            self.7.to_string(),
            self.8.to_string(),
            self.9.to_string(),
        ]
    }
}

impl IntoStrings
    for (
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
    )
{
    fn into_strings(self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
            self.4.to_string(),
            self.5.to_string(),
            self.6.to_string(),
            self.7.to_string(),
            self.8.to_string(),
            self.9.to_string(),
            self.10.to_string(),
        ]
    }
}

impl IntoStrings
    for (
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
    )
{
    fn into_strings(self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
            self.4.to_string(),
            self.5.to_string(),
            self.6.to_string(),
            self.7.to_string(),
            self.8.to_string(),
            self.9.to_string(),
            self.10.to_string(),
            self.11.to_string(),
        ]
    }
}

impl IntoStrings
    for (
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
    )
{
    fn into_strings(self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
            self.4.to_string(),
            self.5.to_string(),
            self.6.to_string(),
            self.7.to_string(),
            self.8.to_string(),
            self.9.to_string(),
            self.10.to_string(),
            self.11.to_string(),
            self.12.to_string(),
        ]
    }
}

impl IntoStrings
    for (
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
    )
{
    fn into_strings(self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
            self.4.to_string(),
            self.5.to_string(),
            self.6.to_string(),
            self.7.to_string(),
            self.8.to_string(),
            self.9.to_string(),
            self.10.to_string(),
            self.11.to_string(),
            self.12.to_string(),
            self.13.to_string(),
        ]
    }
}

impl IntoStrings
    for (
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
    )
{
    fn into_strings(self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
            self.4.to_string(),
            self.5.to_string(),
            self.6.to_string(),
            self.7.to_string(),
            self.8.to_string(),
            self.9.to_string(),
            self.10.to_string(),
            self.11.to_string(),
            self.12.to_string(),
            self.13.to_string(),
            self.14.to_string(),
        ]
    }
}

impl IntoStrings
    for (
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
        &str,
    )
{
    fn into_strings(self) -> Vec<String> {
        vec![
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
            self.3.to_string(),
            self.4.to_string(),
            self.5.to_string(),
            self.6.to_string(),
            self.7.to_string(),
            self.8.to_string(),
            self.9.to_string(),
            self.10.to_string(),
            self.11.to_string(),
            self.12.to_string(),
            self.13.to_string(),
            self.14.to_string(),
            self.15.to_string(),
        ]
    }
}
