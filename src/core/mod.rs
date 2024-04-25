pub trait IntoDomain<D> {
    /// Convert self into the corresponding domain type.
    fn into_domain(self) -> D;
}

pub trait FromDomain<T, D> {
    /// Create an instance of T from the provided domain type.
    fn from_domain(domain: D) -> T;
}