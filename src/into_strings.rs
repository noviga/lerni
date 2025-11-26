/// Trait for types that can be converted into a vector of strings.
pub trait IntoStrings {
    /// Converts self into vector of strings.
    fn into_strings(self) -> Vec<String>;
}

impl IntoStrings for &str {
    fn into_strings(self) -> Vec<String> {
        vec![self.into()]
    }
}

impl IntoStrings for (&str,) {
    fn into_strings(self) -> Vec<String> {
        vec![self.0.into()]
    }
}

impl IntoStrings for (&str, &str) {
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        vec![a.into(), b.into()]
    }
}

impl IntoStrings for (&str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        let (a, b, c) = self;
        vec![a.into(), b.into(), c.into()]
    }
}

impl IntoStrings for (&str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d) = self;
        vec![a.into(), b.into(), c.into(), d.into()]
    }
}

impl IntoStrings for (&str, &str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e) = self;
        vec![a.into(), b.into(), c.into(), d.into(), e.into()]
    }
}

impl IntoStrings for (&str, &str, &str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f) = self;
        vec![a.into(), b.into(), c.into(), d.into(), e.into(), f.into()]
    }
}

impl IntoStrings for (&str, &str, &str, &str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g) = self;
        vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
        ]
    }
}

impl IntoStrings for (&str, &str, &str, &str, &str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g, h) = self;
        vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
        ]
    }
}

impl IntoStrings for (&str, &str, &str, &str, &str, &str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g, h, i) = self;
        vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
        ]
    }
}

impl IntoStrings for (&str, &str, &str, &str, &str, &str, &str, &str, &str, &str) {
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g, h, i, j) = self;
        vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
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
        let (a, b, c, d, e, f, g, h, i, j, k) = self;
        vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
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
        let (a, b, c, d, e, f, g, h, i, j, k, l) = self;
        vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
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
        let (a, b, c, d, e, f, g, h, i, j, k, l, m) = self;
        vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
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
        let (a, b, c, d, e, f, g, h, i, j, k, l, m, n) = self;
        vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
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
        let (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o) = self;
        vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
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
        let (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p) = self;
        vec![
            a.into(),
            b.into(),
            c.into(),
            d.into(),
            e.into(),
            f.into(),
            g.into(),
            h.into(),
            i.into(),
            j.into(),
            k.into(),
            l.into(),
            m.into(),
            n.into(),
            o.into(),
            p.into(),
        ]
    }
}

impl IntoStrings
    for (
        (
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
        ),
        &str,
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.push(b.into());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (&str, &str),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (&str, &str, &str),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (&str, &str, &str, &str),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (&str, &str, &str, &str, &str),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (&str, &str, &str, &str, &str, &str),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (&str, &str, &str, &str, &str, &str, &str),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (&str, &str, &str, &str, &str, &str, &str, &str),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (&str, &str, &str, &str, &str, &str, &str, &str, &str),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (&str, &str, &str, &str, &str, &str, &str, &str, &str, &str),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (
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
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (
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
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (
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
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (
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
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (
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
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
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
        ),
        (
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
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings for String {
    fn into_strings(self) -> Vec<String> {
        vec![self]
    }
}

impl IntoStrings for (String,) {
    fn into_strings(self) -> Vec<String> {
        vec![self.0]
    }
}

impl IntoStrings for (String, String) {
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        vec![a, b]
    }
}

impl IntoStrings for (String, String, String) {
    fn into_strings(self) -> Vec<String> {
        let (a, b, c) = self;
        vec![a, b, c]
    }
}

impl IntoStrings for (String, String, String, String) {
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d) = self;
        vec![a, b, c, d]
    }
}

impl IntoStrings for (String, String, String, String, String) {
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e) = self;
        vec![a, b, c, d, e]
    }
}

impl IntoStrings for (String, String, String, String, String, String) {
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f) = self;
        vec![a, b, c, d, e, f]
    }
}

impl IntoStrings for (String, String, String, String, String, String, String) {
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g) = self;
        vec![a, b, c, d, e, f, g]
    }
}

impl IntoStrings
    for (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g, h) = self;
        vec![a, b, c, d, e, f, g, h]
    }
}

impl IntoStrings
    for (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g, h, i) = self;
        vec![a, b, c, d, e, f, g, h, i]
    }
}

impl IntoStrings
    for (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g, h, i, j) = self;
        vec![a, b, c, d, e, f, g, h, i, j]
    }
}

impl IntoStrings
    for (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g, h, i, j, k) = self;
        vec![a, b, c, d, e, f, g, h, i, j, k]
    }
}

impl IntoStrings
    for (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g, h, i, j, k, l) = self;
        vec![a, b, c, d, e, f, g, h, i, j, k, l]
    }
}

impl IntoStrings
    for (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g, h, i, j, k, l, m) = self;
        vec![a, b, c, d, e, f, g, h, i, j, k, l, m]
    }
}

impl IntoStrings
    for (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g, h, i, j, k, l, m, n) = self;
        vec![a, b, c, d, e, f, g, h, i, j, k, l, m, n]
    }
}

impl IntoStrings
    for (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o) = self;
        vec![a, b, c, d, e, f, g, h, i, j, k, l, m, n, o]
    }
}

impl IntoStrings
    for (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p) = self;
        vec![a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p]
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        String,
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.push(b);
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (String, String),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (String, String, String),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (String, String, String, String),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (String, String, String, String, String),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (String, String, String, String, String, String),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (String, String, String, String, String, String, String),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}

impl IntoStrings
    for (
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
        (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ),
    )
{
    fn into_strings(self) -> Vec<String> {
        let (a, b) = self;
        let mut result = a.into_strings();
        result.extend(b.into_strings());
        result
    }
}
