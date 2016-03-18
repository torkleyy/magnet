use std::io::Read;

use install;

pub struct PackageName {
    pub name: String,
    pub variant: String,
}

pub struct PackageQuery {
    pub version: VersionCondition,
    pub name: PackageName,
}

pub enum VersionCondition {
    GreaterThan(u64),
    Not(Box<VersionCondition>),
    And(Box<VersionCondition>, Box<VersionCondition>),
}

impl VersionCondition {
    pub fn serialize(&self) -> String {
        match self {
            &VersionCondition::GreaterThan(ref v) => v.to_string(),
            &VersionCondition::Not(ref v) => "!".to_owned() + &v.serialize(),
            &VersionCondition::And(ref v, ref w) => "[".to_owned() + &v.serialize() + "&" + &v.serialize() + "]",
        }
    }
}

pub struct Package {
    pub name: PackageName,
    pub dependencies: Vec<PackageQuery>,
    pub extractor: install::Extractor,
}

impl Package {
    pub fn load<R: Read>(data: R) -> Result<Package, ()> {
        unimplemented!();
    }
}
