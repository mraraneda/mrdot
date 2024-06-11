use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Dotfiles {
    pub base_path: String,
    pub remove_on_conflict: bool,
    pub targets: Vec<Target>
}

#[derive(Deserialize, Debug)]
pub struct Target {
    pub application: String,
    pub elements: Vec<Element>,
}

#[derive(Deserialize, Debug)]
pub struct Element {
    pub path: String,
}
