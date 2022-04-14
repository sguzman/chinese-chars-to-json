use crate::types::china::China;
use crate::types::json::Person;

use crate::declare::skeleton2::Skeleton;
use crate::types::obj2::Prog2;

impl Skeleton for Prog2 {
    fn extract(content: String) -> China {
        let content = content.split('\t');
        let content = content.map(|s| s.to_string());
        let content: Vec<String> = content.collect();

        let fst: String = content[0].clone();
        let snd: String = content[1].clone();
        let lst: String = content[2].clone();

        (fst, snd, lst)
    }

    fn json(china: China) -> Person {
        Person {
            character: china.0,
            pronounce: china.1,
            definition: china.2,
        }
    }
}
