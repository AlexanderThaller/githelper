//! Errors that can be emitted by this crate.

use git2::Error as Git2Error;

/// The error type containing errors.
#[derive(Debug)]
pub enum Error {
    /// Error when repository can not be initalized.
    RepositoryInit(Git2Error),

    /// Error when repository can not be opened.
    RepositoryOpen(Git2Error),

    /// Error when index can not be opened.
    IndexOpen(Git2Error),

    /// Error when file status can not be determined.
    FileStatus(Git2Error),

    /// Error while trying to update given paths in the index.
    IndexUpdateAll(Git2Error),

    /// Error while writing index.
    IndexWrite(Git2Error),

    /// Error while getting repostiroy signature.
    RepositorySignature(Git2Error),

    /// Error while writing index tree.
    IndexWriteTree(Git2Error),

    /// Error while finding repository tree.
    RepositoryFindTree(Git2Error),

    /// Error while commiting to repository.
    RepositoryCommit(Git2Error),

    /// Error while trying to resolve repository HEAD.
    RepositoryHead(Git2Error),

    /// Error while trying to peel a reference into a commit.
    ReferencePeelToCommit(Git2Error),

    /// Error while trying to peel a reference into a commit.
    IndexAddPath(Git2Error),

    /// Error while stripping repository prefix from path when trying to walk
    /// through the repository.
    StripRepositoryPrefix(std::path::StripPrefixError),
}
