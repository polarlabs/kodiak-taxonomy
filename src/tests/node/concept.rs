use crate::Identity;
use uuid::Uuid;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Concept {
    id: Uuid,
    #[allow(dead_code)]
    name: String,
}

impl Concept {
    pub fn new(name: &str) -> Concept {
        Concept {
            id: Uuid::new_v4(),
            name: name.to_owned(),
        }
    }
}

impl Identity<Uuid> for Concept {
    fn id(&self) -> Uuid {
        self.id
    }
}
