use hyper::Client;
use hyper::Url;
use hyper::header::Connection;

use package::{Package, PackageQuery};

pub struct Index {
    url: Url,
}

pub enum QueryError {
    InvalidData,
    NotFound,
}

impl Index {
    pub fn lookup(&self, pack: &PackageQuery) -> Result<Package, QueryError> {
        let mut url = self.url.clone();

        url.query_pairs_mut().clear().extend_pairs(&[
            ("name", &pack.name.name),
            ("variant", &pack.name.variant),
            //("version", &pack.version.serialize())
        ]);

		let mut client = Client::new();

		let mut res = client.get(url).header(Connection::close()).send().map_err(|_| QueryError::NotFound)?;

        Package::load(&mut res).map_err(|_| QueryError::InvalidData)
    }
}
