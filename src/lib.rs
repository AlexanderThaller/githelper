//! Helper crate around git2 with functions for common tasks related to git
//! repositories.

#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod error;

pub use crate::error::Error;

use std::path::Path;

use git2::{
    self,
    Repository,
};
use walkdir::{
    DirEntry,
    WalkDir,
};

/// Commit current stage with given commit message.
pub fn commit<P: AsRef<Path>>(repo_path: P, message: &str) -> Result<(), Error> {
    let repository = Repository::open(&repo_path).map_err(Error::RepositoryOpen)?;
    let mut index = repository.index().map_err(Error::IndexOpen)?;
    let oid = index.write_tree().map_err(Error::IndexWriteTree)?;

    let mut parent_commit = vec![];
    if let Ok(head) = repository.head() {
        if let Ok(head_commit) = head.peel_to_commit() {
            parent_commit.push(head_commit);
        };
    };

    let tree = repository
        .find_tree(oid)
        .map_err(Error::RepositoryFindTree)?;

    let signature = repository.signature().map_err(Error::RepositorySignature)?;

    repository
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parent_commit.iter().collect::<Vec<_>>(),
        )
        .map_err(Error::RepositoryCommit)?;

    Ok(())
}

/// Create a new nonbare git repository in the given path.
pub fn init<P: AsRef<Path>>(repo_path: P) -> Result<(), Error> {
    let _ = Repository::init(&repo_path).map_err(Error::RepositoryInit)?;

    Ok(())
}

/// Push commits to upstream. By default this will try to push to origin. If
/// origin does not exist this function will fail with
/// Error::NoOriginConfigured.
pub fn push_to_origin<P: AsRef<Path>>(repo_path: P) -> Result<(), Error> {
    let repository = Repository::open(&repo_path).map_err(Error::RepositoryOpen)?;
    let mut remote = repository
        .find_remote("origin")
        .map_err(|_| Error::NoOriginConfigured)?;

    remote
        .connect(git2::Direction::Push)
        .map_err(Error::RemoteConnect)?;

    remote
        .push(&["refs/heads/master:refs/heads/master"], None)
        .map_err(Error::RemotePush)?;

    Ok(())
}

/// Stage given paths in the repository. Paths have to be relative to the
/// repo_path.
pub fn stage<P: AsRef<Path>, F: AsRef<Path>>(repo_path: P, paths: &[F]) -> Result<(), Error> {
    let repository = Repository::open(&repo_path).map_err(Error::RepositoryOpen)?;
    let mut index = repository.index().map_err(Error::IndexOpen)?;

    paths
        .iter()
        .filter(|path| {
            let fullpath = repo_path.as_ref().join(path);
            fullpath.exists() && fullpath.is_file()
        })
        .filter(|entry| entry.as_ref() == std::path::PathBuf::from(".git"))
        .map(|path| index.add_path(path.as_ref()).map_err(Error::IndexAddPath))
        .collect::<Result<_, Error>>()?;

    index.write().map_err(Error::IndexWrite)?;

    Ok(())
}

/// Stage all paths in the repository.
pub fn stage_all<P: AsRef<Path>>(repo_path: P) -> Result<(), Error> {
    let paths = WalkDir::new(&repo_path)
        .into_iter()
        .filter_map(|v| v.ok())
        .filter(|entry| entry.path() != repo_path.as_ref())
        .map(|entry| {
            entry
                .into_path()
                .strip_prefix(&repo_path)
                .map(|path| path.to_path_buf())
                .map_err(Error::StripRepositoryPrefix)
        })
        .collect::<Result<Vec<_>, _>>()?;

    stage(repo_path, &paths)?;

    Ok(())
}

/// Return status of the repository.
pub fn status<P: AsRef<Path>>(_repo_path: P) -> Result<(), Error> {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use std::{
        fs,
        path::PathBuf,
    };
    use tempfile::tempdir;

    #[test]
    fn init() {
        let dir = tempdir().unwrap();
        super::init(&dir).unwrap();
    }

    #[test]
    fn stage_nonexistent_file() {
        let dir = tempdir().unwrap();
        super::init(&dir).unwrap();

        let files = vec![PathBuf::from("file_does_not_exist")];
        super::stage(&dir, &files).unwrap();
    }

    #[test]
    fn stage_directory() {
        let dir = tempdir().unwrap();
        super::init(&dir).unwrap();

        let subdir = dir.path().join("subdir");
        std::fs::create_dir_all(&subdir).unwrap();

        let files = vec!["subdir"];
        super::stage(&dir, &files).unwrap();
    }

    #[test]
    fn stage_directory_with_subfile() {
        let dir = tempdir().unwrap();
        super::init(&dir).unwrap();

        let subdir = dir.path().join("subdir");
        std::fs::create_dir_all(&subdir).unwrap();

        let subfile = subdir.join("subfile");
        fs::write(&subfile, "first data").unwrap();

        let files = vec!["subdir", "subdir/subfile"];
        super::stage(&dir, &files).unwrap();
    }

    #[test]
    fn stage_existing_new_file() {
        let dir = tempdir().unwrap();
        super::init(&dir).unwrap();

        let first_file = dir.path().join("first_file");
        fs::write(&first_file, "first data").unwrap();

        let files = vec!["first_file"];
        super::stage(&dir, &files).unwrap();
    }

    #[test]
    fn commit_file_new_repo() {
        let dir = tempdir().unwrap();
        super::init(&dir).unwrap();

        let first_file = dir.path().join("first_file");
        fs::write(&first_file, "first data").unwrap();

        let files = vec!["first_file"];
        super::stage(&dir, &files).unwrap();

        super::commit(&dir, "Added first_file").unwrap();
    }

    #[test]
    fn commit_file_existing_repo() {
        let dir = tempdir().unwrap();
        super::init(&dir).unwrap();

        let first_file = dir.path().join("first_file");
        fs::write(&first_file, "first data").unwrap();

        let files = vec!["first_file"];
        super::stage(&dir, &files).unwrap();

        super::commit(&dir, "Added first_file").unwrap();

        let second_file = dir.path().join("second_file");
        fs::write(&second_file, "second data").unwrap();

        let files = vec!["second_file"];
        super::stage(&dir, &files).unwrap();

        super::commit(&dir, "Added second_file").unwrap();
    }

    #[test]
    fn commit_files_new_repo() {
        let dir = tempdir().unwrap();
        super::init(&dir).unwrap();

        let files = vec!["first_file", "second_file", "third_file"];
        for file in &files {
            let path = dir.path().join(file);
            fs::write(&path, file).unwrap();
        }

        super::stage(&dir, &files).unwrap();
        super::commit(&dir, "Added files").unwrap();
    }

    #[test]
    fn stage_all_files() {
        let dir = tempdir().unwrap();
        super::init(&dir).unwrap();

        let files = vec!["first_file", "second_file", "third_file"];
        for file in &files {
            let path = dir.path().join(file);
            fs::write(&path, file).unwrap();
        }

        super::stage_all(&dir).unwrap();
    }

    #[test]
    fn commit_all_files() {
        let dir = tempdir().unwrap();
        super::init(&dir).unwrap();

        let files = vec!["first_file", "second_file", "third_file"];
        for file in &files {
            let path = dir.path().join(file);
            fs::write(&path, file).unwrap();
        }

        super::stage_all(&dir).unwrap();
        super::commit(&dir, "added all files").unwrap();
    }

    #[test]
    #[should_panic]
    fn status_not_a_repository() {
        let dir = tempdir().unwrap();
        super::status(&dir).unwrap();
    }
}
