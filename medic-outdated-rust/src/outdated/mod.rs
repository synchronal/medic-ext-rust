use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Dependency {
    pub latest: String,
    pub name: String,
    pub project: String,
}

#[derive(Debug, Deserialize)]
pub struct OutdatedInfo {
    pub dependencies: Vec<Dependency>,
}
