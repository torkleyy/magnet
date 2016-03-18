use trust::Trustee;
use index::Index;
use package::PackageName;

use std::collections::HashMap;

type PackageId = u16;

/// The system.
///
/// This keeps track of the installed, uninstalled, used, modified etc. packages and their
/// associated information.
pub struct System {
    /// The installed packages.
    installed: HashMap<PackageId, LocalPackage>,
    /// The package indexes.
    ///
    /// Sorted after reliability.
    indexes: Vec<Index>,
    /// Trusted keys.
    ///
    /// Sorted after trust.
    trustees: Vec<Trustee>,
}

/// A local, installed package.
pub struct LocalPackage {
    /// Name of the package.
    name: PackageName,
    /// Number of packages this package is being dependency for.
    dependency_for: u16,
    /// The dependencies of this package.
    dependencies: Vec<PackageId>,
}

