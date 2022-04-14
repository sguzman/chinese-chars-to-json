use crate::types::china::China;
use crate::types::json::Person;

pub trait Skeleton {
    fn extract(content: String) -> China;
    fn json(china: China) -> Person;

    fn f(content: String) -> Person {
        Self::json(Self::extract(content))
    }
}
