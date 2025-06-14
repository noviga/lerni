/// Trait for types that can be converted into a vector of strings.
pub trait IntoStrings {
    /// Converts self into vector of strings.
    fn into_strings(self) -> Vec<String>;
}

macro_rules! into_strings {
    ($($ty:ident),*) => {
        impl<$($ty),*> IntoStrings for ($($ty,)*)
        where
            $($ty: Into<String>),*,
        {
            fn into_strings(self) -> Vec<String> {
                #[allow(non_snake_case)]
                let ($($ty,)*) = self;
                vec![$($ty.into(),)*]
            }
        }
    }
}

into_strings!(A);
into_strings!(A, B);
into_strings!(A, B, C);
into_strings!(A, B, C, D);
into_strings!(A, B, C, D, E);
into_strings!(A, B, C, D, E, F);
into_strings!(A, B, C, D, E, F, G);
into_strings!(A, B, C, D, E, F, G, H);
into_strings!(A, B, C, D, E, F, G, H, I);
into_strings!(A, B, C, D, E, F, G, H, I, J);
into_strings!(A, B, C, D, E, F, G, H, I, J, K);
into_strings!(A, B, C, D, E, F, G, H, I, J, K, L);
into_strings!(A, B, C, D, E, F, G, H, I, J, K, L, M);
into_strings!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
into_strings!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
into_strings!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
into_strings!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
into_strings!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
into_strings!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
into_strings!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
into_strings!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U
);
into_strings!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V
);
into_strings!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W
);
into_strings!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X
);
into_strings!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y
);
into_strings!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z
);
