use crate::declare::skeleton::Skeleton;
use crate::types::obj::Prog;

impl Skeleton for Prog {
    fn contents(path: String) -> String {
        std::fs::read_to_string(path).expect("No file :(")
    }

    fn list(content: String) -> Vec<String> {
        content.lines().map(|s| s.to_string()).collect()
    }
}
