use std::io::{self, Read};

use install;

#[derive(Clone, PartialEq, Hash, Debug)]
pub struct PackageName {
    pub name: String,
    pub variant: String,
}

/// A package query.
///
/// In other words, a package name together with a version condition.
#[derive(Clone, PartialEq, Hash, Debug)]
pub struct PackageQuery {
    pub version: VersionCondition,
    pub name: PackageName,
}

#[derive(Clone, PartialEq, Hash, Debug)]
pub enum VersionCondition {
    GreaterThan(u64),
    Not(Box<VersionCondition>),
    And(Box<VersionCondition>, Box<VersionCondition>),
}

#[derive(Debug)]
pub enum DeserializationError {
    IoError(io::Error),
    UnknownType(u8),
}

impl From<io::Error> for DeserializationError {
    fn from(from: io::Error) -> DeserializationError {
        DeserializationError::IoError(from)
    }
}

impl VersionCondition {
    /// Serialize this verison condition.
    ///
    /// The serialization is quite simple: In case of `GreaterThan`, we serialize the number by
    /// using a base 256 big-endian representation of the integer. When this 8 bytes are over,
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
    /// Starting at the top, we go to the left child and then the right child. Since the top node
    /// is just a basic and, we start with 0x0 (AND), then it consumes two nodes, the left one and
    /// the right one. The left child is a NAND (0x1 0x0), which again consumes two conditions, the
    /// left one and the right one. The left child in this case is a NOT (0x1), which will consume
    /// one node, which is of type 0x2 (greater than), and then the 8 byte integral value.
    ///
    /// So if we print it all out:
    ///
    /// ```
    /// 00 (AND)
    /// 01 (NOT)
    /// 00 (AND)
    /// 03 (NOT)
    /// 02 (GREATER_THAN)
    /// 00 (integer)
    /// 00
    /// 00
    /// 00
    /// 00
    /// 00
    /// 00
    /// 01
    /// [We now go to the RHS of the NAND]
    /// 02 (GREAT_THAN)
    /// 00 (integer)
    /// 00
    /// 00
    /// 00
    /// 00
    /// 00
    /// 00
    /// FF
    /// [We now go to the RHS of the AND]
    /// ...
    /// ```
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            &VersionCondition::GreaterThan(v) => vec![
                0x2,
                (v >> 56) as u8,
                (v >> 48) as u8,
                (v >> 40) as u8,
                (v >> 32) as u8,
                (v >> 24) as u8,
                (v >> 16) as u8,
                (v >> 8) as u8,
                v as u8,
            ],
            &VersionCondition::Not(ref v) => {
                let mut ser = v.serialize();
                ser.insert(0, 0x1);
                ser
            }
            &VersionCondition::And(ref v, ref w) => {
                let mut ser = vec![0x0];
                ser.extend_from_slice(&v.serialize());
                ser.extend_from_slice(&w.serialize());
                ser
            },
        }
    }

    /// Deserialize the byte encoding of this version condition.
    pub fn deserialize<R: Read>(read: &mut R) -> Result<VersionCondition, DeserializationError> {
        let mut introducer = [0];

        try!(read.read_exact(&mut introducer));

        match introducer[0] {
            0x2 => {
                let mut num = [0; 8];
                try!(read.read_exact(&mut num));
                Ok(VersionCondition::GreaterThan(num.iter().fold(0, |x, &y| x << 8 + y as u64)))
            },
            0x1 => Ok(VersionCondition::Not(box try!(VersionCondition::deserialize(&mut *read)))),
            0x0 => Ok(VersionCondition::And(
                box try!(VersionCondition::deserialize(&mut *read)),
                box try!(VersionCondition::deserialize(&mut *read)),
            )),
            n => Err(DeserializationError::UnknownType(n)),
        }
    }
}

#[derive(Clone, PartialEq, Hash, Debug)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize() {
        fn assert_ser(v: VersionCondition) {
            let mut ser = v.serialize();
            assert_eq!(VersionCondition::deserialize(&mut ser.as_slice()).unwrap(), v);
        }
        assert_ser(VersionCondition::GreaterThan(456));
        assert_ser(VersionCondition::Not(box VersionCondition::GreaterThan(456)));
        assert_ser(VersionCondition::And(box VersionCondition::Not(box VersionCondition::GreaterThan(456)), box VersionCondition::GreaterThan(65)));
    }
}
