pub trait Skeleton {
    fn contents(path: String) -> String;
    fn list(content: String) -> Vec<String>;

    fn f(path: String) -> Vec<String> {
        Self::list(Self::contents(path))
    }
}
