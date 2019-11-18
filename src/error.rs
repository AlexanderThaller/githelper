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
    IndexAddPath(Git2Error),

    /// Error while stripping repository prefix from path when trying to walk
    /// through the repository.
    StripRepositoryPrefix(std::path::StripPrefixError),

    /// Error when trying to push to origin but not origin is defined.
    NoOriginConfigured,

    /// Error when connecting to remote.
    RemoteConnect(Git2Error),

    /// Error when pushing to remote.
    RemotePush(Git2Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match self {
            FileStatus(err) => write!(f, "can not determine file status: {}", err),
            IndexAddPath(err) => write!(f, "can not add path to index: {}", err),
            IndexOpen(err) => write!(f, "can not open index: {}", err),
            IndexWrite(err) => write!(f, "can not write index: {}", err),
            IndexWriteTree(err) => write!(f, "can not write index tree: {}", err),
            NoOriginConfigured => write!(f, "no origin configured"),
            RemoteConnect(err) => write!(f, "can not connet to remote: {}", err),
            RemotePush(err) => write!(f, "can not push to remote: {}", err),
            RepositoryCommit(err) => write!(f, "can not commit to repository: {}", err),
            RepositoryFindTree(err) => write!(f, "can not find tree in repository: {}", err),
            RepositoryHead(err) => write!(f, "can not find head of repository: {}", err),
            RepositoryInit(err) => write!(f, "can not init repository: {}", err),
            RepositoryOpen(err) => write!(f, "can not open repository: {}", err),
            RepositorySignature(err) => write!(f, "can not get signature from repository: {}", err),
            StripRepositoryPrefix(err) => {
                write!(f, "can not strip repository path prefix: {}", err)
            }
        }
    }
}
