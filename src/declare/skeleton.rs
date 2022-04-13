use std::path::Path;
use crate::types::china::China;
use crate::types::json::Person;

pub trait Skeleton {
 fn dir(path: String) -> Path;
 fn files(path: Path) -> Vec<String>;
 fn file(path: String) -> Path;
 fn contents(path: Path) -> String;
 fn extract(content: String) -> China;
 fn json(china: China) -> Person;
}