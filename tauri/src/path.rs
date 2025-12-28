use std::cmp::Ordering;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

fn get_children<F, G, T>(
    parent: &Path,
    predicate: &F,
    sort_elem: &G,
) -> Result<Vec<PathBuf>, String>
where
    F: Fn(&PathBuf) -> bool,
    G: Fn(&PathBuf) -> T,
    T: Ord,
{
    match read_dir(parent) {
        Ok(entries) => {
            let mut paths: Vec<PathBuf> = entries
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(predicate)
                .collect();
            paths.sort_unstable_by(|p1, p2| sort_elem(p1).partial_cmp(&sort_elem(p2)).unwrap());
            Ok(paths)
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn get_child_files<F, T>(parent: &Path, sort_elem: &F) -> Result<Vec<PathBuf>, String>
where
    F: Fn(&PathBuf) -> T,
    T: Ord,
{
    get_children(parent, &|path| path.is_file(), sort_elem)
}

pub fn get_child_directories<F, T>(parent: &Path, sort_elem: &F) -> Result<Vec<PathBuf>, String>
where
    F: Fn(&PathBuf) -> T,
    T: Ord,
{
    get_children(parent, &|path| path.is_dir(), sort_elem)
}

pub fn next_directory<F, T>(path: &Path, sort_elem: &F) -> Result<PathBuf, String>
where
    F: Fn(&PathBuf) -> T,
    T: Ord,
{
    // Return child directory if exists
    if let Ok(dirs) = get_child_directories(&path, sort_elem) {
        if let Some(next_dir) = dirs.first() {
            return Ok(next_dir.to_path_buf());
        }
    }

    let mut current = path;
    while let Some(parent) = current.parent() {
        match get_child_directories(&parent, sort_elem) {
            Ok(dirs) => {
                if let Some(cursor) = dirs.iter().position(|dir| {
                    sort_elem(dir).partial_cmp(&sort_elem(&current.to_path_buf()))
                        == Some(Ordering::Greater)
                }) {
                    return Ok(dirs[cursor].clone());
                }
                current = parent;
            }
            Err(err) => return Err(err),
        }
    }

    Err("No directory is left".to_string())
}

pub fn prev_directory<F, T>(path: &Path, sort_elem: &F) -> Result<PathBuf, String>
where
    F: Fn(&PathBuf) -> T,
    T: Ord,
{
    match path.parent() {
        Some(parent) => match get_child_directories(&parent, sort_elem) {
            Ok(dirs) => match dirs.iter().rposition(|dir| {
                sort_elem(dir).partial_cmp(&sort_elem(&path.to_path_buf())) == Some(Ordering::Less)
            }) {
                Some(cursor) => {
                    let mut current = dirs[cursor].clone();
                    while let Ok(dirs) = get_child_directories(&current, sort_elem) {
                        if let Some(dir) = dirs.last() {
                            current = dir.to_path_buf();
                        } else {
                            break;
                        }
                    }
                    Ok(current)
                }
                _ => Ok(parent.to_path_buf()),
            },
            _ => Ok(parent.to_path_buf()),
        },
        _ => Err("No directory is left".to_string()),
    }
}

pub fn get_directory(path: &Path) -> Result<PathBuf, String> {
    if !path.exists() {
        return Err(format!("Path `{:?}` does not exist", path));
    }

    match path.is_dir() {
        true => Ok(path.to_path_buf()),
        false => match path.parent() {
            Some(parent) => Ok(parent.to_path_buf()),
            _ => Err(format!("Cannot get parent directory of `{:?}`", path)),
        },
    }
}

pub trait PathRepository {
    fn files(&self) -> Result<Vec<PathBuf>, String>;
    fn next_directory(&self) -> Result<(), String>;
    fn prev_directory(&self) -> Result<(), String>;
}

pub struct FilePathRepository<F, T>
where
    F: Fn(&PathBuf) -> T,
    T: Ord,
{
    directory: Mutex<PathBuf>,
    sort: F,
}

impl<F, T> FilePathRepository<F, T>
where
    F: Fn(&PathBuf) -> T,
    T: Ord,
{
    pub fn new(path: &Path, sort: F) -> Self {
        Self {
            directory: Mutex::new(path.to_path_buf()),
            sort: sort,
        }
    }
}

impl<F, T> PathRepository for FilePathRepository<F, T>
where
    F: Fn(&PathBuf) -> T,
    T: Ord,
{
    fn files(&self) -> Result<Vec<PathBuf>, String> {
        match self.directory.lock() {
            Ok(directory) => get_child_files(&directory, &self.sort),
            Err(err) => Err(err.to_string()),
        }
    }
    fn next_directory(&self) -> Result<(), String> {
        match self.directory.lock() {
            Ok(mut directory) => {
                *directory = next_directory(&directory, &self.sort)?;
                Ok(())
            }
            Err(err) => Err(err.to_string()),
        }
    }
    fn prev_directory(&self) -> Result<(), String> {
        match self.directory.lock() {
            Ok(mut directory) => {
                *directory = prev_directory(&directory, &self.sort)?;
                Ok(())
            }
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir, File};
    use tempfile::{tempdir, TempDir};

    fn init_test_filesystem() -> TempDir {
        /*
         * test file structure
         * root/ +- a/ +- a/ +- a
         *       |     |     +- b
         *       |     |     +- c
         *       |     +- b/ +- a
         *       |     |     +- b
         *       |     |     +- c
         *       |     +- c/
         *       |     +- d
         *       |
         *       +- b/ +- a/ +- a
         *       |     |
         *       |     +- b/ +- a
         *       |     |
         *       |     +- c/
         *       |
         *       +- c/ +- a
         *             +- b
         *             +- c
         */
        let root_dir = tempdir().unwrap();
        let _ = create_dir(root_dir.path().join("a"));
        let _ = create_dir(root_dir.path().join("a/a"));
        let _ = create_dir(root_dir.path().join("a/b"));
        let _ = create_dir(root_dir.path().join("a/c"));
        let _ = create_dir(root_dir.path().join("b"));
        let _ = create_dir(root_dir.path().join("b/a"));
        let _ = create_dir(root_dir.path().join("b/b"));
        let _ = create_dir(root_dir.path().join("b/c"));
        let _ = create_dir(root_dir.path().join("c"));
        let _ = File::create(root_dir.path().join("a/a/a"));
        let _ = File::create(root_dir.path().join("a/a/b"));
        let _ = File::create(root_dir.path().join("a/a/c"));
        let _ = File::create(root_dir.path().join("a/b/a"));
        let _ = File::create(root_dir.path().join("a/b/b"));
        let _ = File::create(root_dir.path().join("a/b/c"));
        let _ = File::create(root_dir.path().join("a/d"));
        let _ = File::create(root_dir.path().join("b/a/a"));
        let _ = File::create(root_dir.path().join("b/b/a"));
        let _ = File::create(root_dir.path().join("c/a"));
        let _ = File::create(root_dir.path().join("c/b"));
        let _ = File::create(root_dir.path().join("c/c"));
        root_dir
    }

    #[test]
    fn test_get_children() {
        let tmp_dir = init_test_filesystem();

        let sort_elem = |path: &PathBuf| path.clone();

        let parent1 = tmp_dir.path().join("a/a");
        let predicate1 = |_: &_| true;
        let actual1 = get_children(&parent1, &predicate1, &sort_elem);

        assert!(actual1.is_ok());
        assert_eq!(
            actual1.unwrap(),
            vec![
                tmp_dir.path().join("a/a/a"),
                tmp_dir.path().join("a/a/b"),
                tmp_dir.path().join("a/a/c"),
            ],
        );

        let parent2 = tmp_dir.path().join("a");
        let predicate2 = |path: &PathBuf| path.is_dir();
        let actual2 = get_children(&parent2, &predicate2, &sort_elem);

        assert!(actual2.is_ok());
        assert_eq!(
            actual2.unwrap(),
            vec![
                tmp_dir.path().join("a/a"),
                tmp_dir.path().join("a/b"),
                tmp_dir.path().join("a/c"),
            ],
        );

        let _ = tmp_dir.close();
    }

    #[test]
    fn test_get_child_files() {
        let tmp_dir = init_test_filesystem();

        let sort_elem = |path: &PathBuf| path.clone();

        let parent1 = tmp_dir.path().join("a/a");
        let actual1 = get_child_files(&parent1, &sort_elem);

        assert!(actual1.is_ok());
        assert_eq!(
            actual1.unwrap(),
            vec![
                tmp_dir.path().join("a/a/a"),
                tmp_dir.path().join("a/a/b"),
                tmp_dir.path().join("a/a/c"),
            ],
        );

        let parent2 = tmp_dir.path().join("a");
        let actual2 = get_child_files(&parent2, &sort_elem);

        assert!(actual2.is_ok());
        assert_eq!(actual2.unwrap(), vec![tmp_dir.path().join("a/d")]);

        let _ = tmp_dir.close();
    }

    #[test]
    fn test_get_child_directories() {
        let tmp_dir = init_test_filesystem();

        let sort_elem = |path: &PathBuf| path.clone();

        let parent1 = tmp_dir.path().join("b");
        let actual1 = get_child_directories(&parent1, &sort_elem);

        assert!(actual1.is_ok());
        assert_eq!(
            actual1.unwrap(),
            vec![
                tmp_dir.path().join("b/a"),
                tmp_dir.path().join("b/b"),
                tmp_dir.path().join("b/c"),
            ],
        );

        let parent2 = tmp_dir.path().join("a");
        let actual2 = get_child_directories(&parent2, &sort_elem);

        assert!(actual2.is_ok());
        assert_eq!(
            actual2.unwrap(),
            vec![
                tmp_dir.path().join("a/a"),
                tmp_dir.path().join("a/b"),
                tmp_dir.path().join("a/c"),
            ],
        );
        let _ = tmp_dir.close();
    }

    #[test]
    fn test_next_directory() {
        let tmp_dir = init_test_filesystem();

        let sort_elem = |path: &PathBuf| path.clone();

        let path = tmp_dir.path().join("a");
        let actual = next_directory(&path, &sort_elem);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), tmp_dir.path().join("a/a"));

        let path = tmp_dir.path().join("a/a");
        let actual = next_directory(&path, &sort_elem);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), tmp_dir.path().join("a/b"));

        let path = tmp_dir.path().join("a/c");
        let actual = next_directory(&path, &sort_elem);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), tmp_dir.path().join("b"));

        let path = tmp_dir.path().join("c");
        let actual = next_directory(&path, &sort_elem);

        if let Ok(path) = actual {
            assert!(!path.starts_with(&tmp_dir));
        }

        let _ = tmp_dir.close();
    }

    #[test]
    fn test_prev_directory() {
        let tmp_dir = init_test_filesystem();

        let sort_elem = |path: &PathBuf| path.clone();

        let path = tmp_dir.path().join("a/a");
        let actual = prev_directory(&path, &sort_elem);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), tmp_dir.path().join("a"));

        let path = tmp_dir.path().join("a/b");
        let actual = prev_directory(&path, &sort_elem);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), tmp_dir.path().join("a/a"));

        let path = tmp_dir.path().join("b");
        let actual = prev_directory(&path, &sort_elem);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), tmp_dir.path().join("a/c"));

        let path = tmp_dir.path();
        let actual = prev_directory(&path, &sort_elem);

        if let Ok(path) = actual {
            assert!(!path.starts_with(&tmp_dir));
        }

        let _ = tmp_dir.close();
    }

    #[test]
    fn test_get_directory() {
        let tmp_dir = init_test_filesystem();

        let path = tmp_dir.path().join("a/a/a");
        let actual = get_directory(&path);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), tmp_dir.path().join("a/a"));

        let path = tmp_dir.path().join("a/a");
        let actual = get_directory(&path);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), tmp_dir.path().join("a/a"));

        let path = tmp_dir.path().join("a/c/a");
        let actual = get_directory(&path);
        assert!(actual.is_err());
    }
}
