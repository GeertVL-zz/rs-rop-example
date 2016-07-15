enum StringError {
    Missing,
    MustNotBeLongerThan(i32),
    DoesntMatchPattern(String),
}