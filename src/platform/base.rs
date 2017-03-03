pub trait Platform {
    fn detect(&self) -> &Self;
}
