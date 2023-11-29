use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Dependency {
    pub compat: String,
    pub latest: String,
    pub name: String,
    pub project: String,
}

#[derive(Debug, Deserialize)]
pub struct OutdatedInfo {
    pub crate_name: String,
    pub dependencies: Vec<Dependency>,
}
