pub fn add_curry<T>(n: T) -> impl Fn(T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    move |x: T| x + n
}

pub fn twice<T, F>(f: F) -> impl Fn(T) -> T 
where
    F: Fn(T) -> T,
{
    move |x: T| f(f(x))
}