use crate::read_lines;

const TOTAL_DISK_SIZE: usize = 70000000;
const MIN_FREE_SPACE: usize = 30000000;
struct Directory {
    name: String,
    children: Vec<Directory>,
    total_size: usize,
}

impl Directory {
    fn add_file(&mut self, _name: &str, size: usize) {
        self.total_size += size;
    }

    fn add_dir(&mut self, name: &str) {
        self.children.push(Directory {
            name: String::from(name),
            children: Vec::new(),
            total_size: 0,
        });
    }

    fn get_child(&mut self, name: &str) -> &mut Directory {
        let has_child = self.children.iter().any(|child| child.name == name);
        if !has_child {
            self.add_dir(name);
        }
        self.children
            .iter_mut()
            .find(|child| child.name == name)
            .unwrap()
    }

    fn get_subchild(&mut self, path: &str) -> &mut Directory {
        if path.is_empty() {
            return self;
        }
        let mut parts = path.split('/').collect::<Vec<&str>>();
        let child_name = parts.remove(0);
        self.get_child(child_name)
            .get_subchild(parts.join("/").as_str())
    }
}

struct FileSystem {
    root: Directory,
    current_path: String,
}

impl FileSystem {
    fn new() -> FileSystem {
        FileSystem {
            root: Directory {
                name: String::from("root"),
                children: Vec::new(),
                total_size: 0,
            },
            current_path: String::from(""),
        }
    }

    fn cd(&mut self, dir_name: &str) {
        let mut path_list = self
            .current_path
            .split('/')
            .filter(|&x| !x.is_empty())
            .collect::<Vec<&str>>();

        match dir_name {
            "/" => path_list = vec![],
            ".." => {
                path_list.pop();
            }
            _ => path_list.push(dir_name),
        }
        self.current_path = path_list.join("/");
    }

    fn current_dir(&mut self) -> &mut Directory {
        self.root.get_subchild(self.current_path.as_str())
    }
}

#[allow(dead_code)]
fn solve(filename: &str, part1: bool) -> usize {
    let lines = read_lines(filename)
        .unwrap()
        .flatten()
        .collect::<Vec<String>>();

    let mut fs = FileSystem::new();

    for line in lines.iter().skip(1) {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                fs.cd(parts[2]);
                continue;
            }
            if parts[1] == "ls" {
                continue;
            }
        } else if parts[0] == "dir" {
            fs.current_dir().add_dir(parts[1])
        } else if let Ok(file_size) = parts[0].parse::<usize>() {
            fs.current_dir().add_file(parts[1], file_size);
        }
    }

    fix_sizes(&mut fs.root);
    let sizes = get_folder_sizes(&mut fs.root);

    if part1 {
        sizes.iter().filter(|&x| *x < 100000).sum()
    } else {
        let currently_free = TOTAL_DISK_SIZE - fs.root.total_size;
        let to_delete = MIN_FREE_SPACE - currently_free;
        sizes.into_iter().filter(|&x| x >= to_delete).min().unwrap()
    }
}

fn fix_sizes(dir: &mut Directory) -> usize {
    let mut sizes = vec![];
    for child in &mut dir.children {
        sizes.push(fix_sizes(child));
    }
    dir.total_size += sizes.iter().sum::<usize>();
    dir.total_size
}

fn get_folder_sizes(dir: &mut Directory) -> Vec<usize> {
    let mut sizes = vec![];
    for child in &mut dir.children {
        sizes.append(&mut get_folder_sizes(child));
    }
    sizes.push(dir.total_size);
    sizes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        assert_eq!(solve("7.test.in", true), 95437);
        assert_eq!(solve("7.test.in", false), 24933642);

        println!("Answer 7 pt.1: {}", solve("7.in", true));
        println!("Answer 7 pt.2: {}", solve("7.in", false));
    }
}
