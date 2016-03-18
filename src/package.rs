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
    /// Serialize this verison condition.
    ///
    /// The serialization is quite simple: In case of `GreaterThan`, we serialize the number by
    /// using a base 256 little-endian representation of the integer. When this 8 bytes are over,
    /// the next byte denotes if `Not` should be applied to the previous condition. If this byte is
    /// one, then `Not` is applied, if not, it will progress to the next `GreaterThan` version
    /// number.
    ///
    /// For the sake of an example:
    ///
    /// Take a tree like,
    ///
    /// ```
    ///       ___&___
    ///      /       \
    ///      ¬      ...
    ///      |
    ///    __&__
    ///   /     \
    ///   ¬     |
    ///   |   >0xFF
    ///   |
    /// >0x01
    /// ```
    ///
    /// Serialization is done recursively:
    ///
    /// Starting at the top, we go to the left child and then the right child. There are two unary
    /// operators which can be done NOT and NOP. NOT (¬) is encoded as 0x1, NON is encoded as 0x0.
    ///
    /// We repeat by walking down the tree. The leaf can be terminated using ~0 TODO
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            &VersionCondition::GreaterThan(ref v) => vec![
                v & 256,
                v << 8 & 256,
                v << 16 & 256,
                v << 24 & 256,
                v << 32 & 256,
                v << 40 & 256,
                v << 48 & 256,
                v << 56
            ],
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
    pub fn load<R: Read>(data: &mut R) -> Result<Package, ()> {
        unimplemented!();
    }
}
