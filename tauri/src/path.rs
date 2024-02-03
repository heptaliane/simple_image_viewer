use std::path::PathBuf;

pub struct PathProvider {
    paths: Vec<PathBuf>,
    cursor: usize,
}

impl PathProvider {
    pub fn new(filenames: &Vec<String>) -> Self {
        Self {
            paths: filenames
                .iter()
                .map(|filename| PathBuf::from(filename))
                .collect(),
            cursor: 0,
        }
    }

    pub fn get(&self) -> Option<String> {
        match self.paths.get(self.cursor) {
            Some(path) => match path.to_str() {
                Some(path_str) => Some(path_str.to_string()),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn move_to(&mut self, position: usize) {
        self.cursor = position;
    }

    pub fn move_next(&mut self) {
        match self.cursor + 1 {
            i if i >= self.paths.len() => (),
            i => self.move_to(i),
        }
    }

    pub fn move_prev(&mut self) {
        match self.cursor {
            0 => (),
            _ => self.move_to(self.cursor - 1),
        }
    }
}
