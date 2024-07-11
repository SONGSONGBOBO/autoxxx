use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Role {
    name: Box<str>,
    skill: Box<str>
}

