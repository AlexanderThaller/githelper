extern crate git2;

use git2::{Repository, Error, ObjectType};
use std::path::Path;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub fn get_current_commitid_for_repo(folder: &Path) -> Result<String, Error> {
    let repo = try!(Repository::open(&folder));
    let head = try!(repo.head());
    let head_name = try!(head.resolve());
    let commit = try!(head_name.peel(ObjectType::Commit));
    let id = commit.id();

    Ok(format!("{}", id))
}
