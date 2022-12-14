use aoc_helpers::data_loader::DataLoader;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::iter;
use std::rc::{Rc, Weak};

struct File {
    name: String,
    size: usize,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (file, size={})", self.name, self.size)
    }
}

struct Dir {
    name: String,
    parent: Weak<RefCell<Dir>>,
    dirs: HashMap<String, Rc<RefCell<Dir>>>,
    files: Vec<File>,
}

impl Dir {
    fn new(name: String) -> Dir {
        Dir {
            name,
            parent: Weak::new(),
            dirs: HashMap::new(),
            files: vec![],
        }
    }

    fn to_pretty_string(&self, indent_lvl: usize) -> String {
        let spaces = iter::repeat(' ').take(indent_lvl * 2).collect::<String>();

        let mut res = String::new();
        write!(
            res,
            "{}- {} (dir, size={})\n",
            spaces.clone(),
            self.name,
            self.dir_size()
        )
        .unwrap();

        for file in self.files.iter() {
            write!(res, "{}  - {}\n", spaces.clone(), file).unwrap();
        }

        for (_, dir) in self.dirs.iter() {
            let dir_str: String = dir.borrow().to_pretty_string(indent_lvl + 1);
            res.push_str(dir_str.as_str());
        }

        res
    }

    fn dir_size(&self) -> usize {
        let mut res = 0;
        res += self.files.iter().map(|f| f.size).sum::<usize>();
        res += self
            .dirs
            .iter()
            .map(|(_, dir)| dir.borrow().dir_size())
            .sum::<usize>();
        res
    }

    fn flatten_sub_dirs(&self) -> Vec<Rc<RefCell<Dir>>> {
        let mut res = vec![];

        for (_, dir) in self.dirs.iter() {
            res.push(dir.clone());
            let mut sub_dirs = dir.borrow().flatten_sub_dirs();
            res.append(&mut sub_dirs);
        }

        res
    }
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.to_pretty_string(0))
    }
}

#[derive(Debug)]
enum Cmd {
    CdParent,
    CdLs(String, Vec<String>),
}

fn get_cmds(loader: &DataLoader) -> Vec<Cmd> {
    let mut res = Vec::new();

    let mut iter = loader.iter().peekable();

    while let Some(line) = iter.next() {
        if line == "$ cd .." {
            res.push(Cmd::CdParent);
        } else if line.starts_with("$ cd ") {
            let dir_name = line["$ cd ".len()..].trim().to_string();
            let mut out_lines: Vec<String> = vec![];

            if Some(&"$ ls".to_string()) != iter.next() {
                panic!("expected ls")
            }

            while let Some(tmp_peek) = iter.peek() {
                if tmp_peek.starts_with("$ ") {
                    break;
                }
                let out_line = iter.next().unwrap();

                out_lines.push(out_line.to_string());
            }

            res.push(Cmd::CdLs(dir_name, out_lines))
        }
    }

    res
}

fn build_fs(cmds: Vec<Cmd>) -> Rc<RefCell<Dir>> {
    let root;
    if let Cmd::CdLs(name, lines) = cmds.first().unwrap() {
        root = create_dir(name, &lines);
        root.borrow_mut().parent = Rc::downgrade(&root);
    } else {
        panic! {"expect ls cmd"};
    }

    let mut current = root.clone();
    for cmd in cmds.iter().skip(1) {
        match cmd {
            Cmd::CdParent => {
                let parent = current.borrow().parent.upgrade().unwrap();
                current = parent;
            }
            Cmd::CdLs(name, lines) => {
                let dir = create_dir(name, lines);
                dir.borrow_mut().parent = Rc::downgrade(&current);
                current.borrow_mut().dirs.insert(name.clone(), dir.clone());
                current = dir;
            }
        }
    }

    root
}

fn create_dir(name: &String, lines: &Vec<String>) -> Rc<RefCell<Dir>> {
    let mut dir = Dir::new(name.to_string());
    for line in lines.iter() {
        if line.starts_with("dir") {
            let dir_name = line.split_once(' ').unwrap().1;
            dir.dirs.insert(
                dir_name.to_string(),
                Rc::new(RefCell::new(Dir::new(dir_name.to_string()))),
            );
        } else {
            let size_name = line.split_once(' ').unwrap();
            dir.files.push(File {
                name: size_name.1.to_string(),
                size: size_name.0.parse::<usize>().unwrap(),
            })
        }
    }
    Rc::new(RefCell::new(dir))
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let cmds = get_cmds(loader);
    let fs = build_fs(cmds);

    let mut all_dirs = fs.borrow().flatten_sub_dirs();
    all_dirs.push(fs.clone());

    let res = all_dirs
        .iter()
        .map(|dir| dir.borrow().dir_size())
        .filter(|size| size <= &100000)
        .sum::<usize>();

    Ok(res.to_string())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let cmds = get_cmds(loader);
    let fs = build_fs(cmds);

    let mut all_dirs = fs.borrow().flatten_sub_dirs();
    all_dirs.push(fs.clone());

    let free_space = 70000000 - fs.borrow().dir_size();
    let space_to_free = 30000000 - free_space;

    let res = all_dirs
        .iter()
        .map(|dir| dir.borrow().dir_size())
        .filter(|size| *size >= space_to_free)
        .min()
        .unwrap();

    Ok(res.to_string())
}
