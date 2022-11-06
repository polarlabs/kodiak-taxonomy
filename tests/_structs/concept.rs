use kodiak_taxonomy::Identity;

use uuid::Uuid;

#[derive(Clone, Debug)]
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

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl Identity<Uuid> for Concept {
    fn id(&self) -> Uuid {
        self.id
    }
}
