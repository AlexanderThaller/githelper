use log::{
    debug,
    trace,
};
use std::{
    io::{
        Error,
        ErrorKind,
        Result,
    },
    path::Path,
    process::Command,
};

pub fn get_current_commitid_for_repo(folder: &Path) -> Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .current_dir(folder)
        .output()?;

    trace!("output of current commit command: {:#?}", output);

    if output.status.success() {
        Ok(format!("{}", String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("{}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

pub fn clone(folder: &Path, remote: &str) -> Result<String> {
    let output = Command::new("git")
        .arg("clone")
        .arg(remote)
        .arg(folder)
        .output()?;

    trace!("output of clone command: {:#?}", output);

    if output.status.success() {
        Ok(format!("{}", String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("{}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

#[test]
fn test_git_clone() {
    use std::path::PathBuf;
    let out = assert!(clone(PathBuf::from("/tmp/git_test_clone").as_path(), ".").is_ok());
    std::fs::remove_dir_all("/tmp/git_test_clone").unwrap();

    out
}

pub fn add(folder: &Path, file: &Path) -> Result<String> {
    let output = Command::new("git")
        .arg("add")
        .arg(file)
        .current_dir(folder)
        .output()?;

    trace!("output of add command: {:#?}", output);

    if output.status.success() {
        Ok(format!("{}", String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("{}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

pub fn commit(folder: &Path, message: &str) -> Result<String> {
    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .current_dir(folder)
        .output()?;

    trace!("output of commit command: {:#?}", output);

    if output.status.success() {
        Ok(format!("{}", String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("{}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

pub fn status(folder: &Path) -> Result<String> {
    let output = Command::new("git")
        .arg("status")
        .current_dir(folder)
        .output()?;

    trace!("output of init command: {:#?}", output);

    if output.status.success() {
        Ok(format!("{}", String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("{}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

pub fn init(folder: &Path) -> Result<String> {
    let output = Command::new("git")
        .arg("init")
        .current_dir(folder)
        .output()?;

    trace!("output of init command: {:#?}", output);

    if output.status.success() {
        Ok(format!("{}", String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("{}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

pub fn pull(folder: &Path) -> Result<String> {
    debug!("pulling git repo in folder {:#?}", folder);

    let output = Command::new("git")
        .arg("pull")
        .current_dir(folder)
        .output()?;

    trace!("output of pull command: {:#?}", output);

    if output.status.success() {
        Ok(format!("{}", String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("{}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

pub fn push(folder: &Path) -> Result<String> {
    debug!("pushing git repo in folder {:#?}", folder);

    let output = Command::new("git")
        .arg("push")
        .current_dir(folder)
        .output()?;

    trace!("output of push command: {:#?}", output);

    if output.status.success() {
        Ok(format!("{}", String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("{}", String::from_utf8_lossy(&output.stderr)),
        ))
    }
}

pub fn sync(folder: &Path) -> Result<String> {
    pull(folder)?;
    push(folder)
}
