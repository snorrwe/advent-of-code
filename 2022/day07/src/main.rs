use std::{collections::HashMap, path::PathBuf};

#[derive(Default, Debug)]
struct Dir {
    files: HashMap<String, i32>,
    children: HashMap<String, Dir>,
    parent: Option<*mut Dir>,
}

impl Dir {
    fn size(&self) -> i32 {
        let res: i32 = self.files.iter().map(|(_, x)| *x).sum();
        let ch: i32 = self.children.iter().map(|(_, c)| c.size()).sum();
        res + ch
    }

    fn iter_dirs<'a>(&'a self) -> Box<dyn Iterator<Item = &'a Self> + 'a> {
        let it =
            std::iter::once(self).chain(self.children.iter().flat_map(|(_k, v)| v.iter_dirs()));
        Box::new(it)
    }
}
enum Cmd {
    Cd,
    Ls,
}

fn parse_input(input: &str) -> Dir {
    let mut root = Dir::default();
    let mut current_dir = &mut root;
    let mut current_path = PathBuf::from("/");
    let mut cmd = Cmd::Cd;
    for line in input.lines() {
        if line.starts_with("$ cd") {
            cmd = Cmd::Cd;
            let dir = &line[5..];
            match dir.trim() {
                ".." => {
                    current_path = current_path.parent().map(|x| x.to_path_buf()).unwrap();
                    current_dir = unsafe { &mut *current_dir.parent.unwrap() };
                }
                "/" => {
                    current_dir = &mut root;
                    current_path = PathBuf::from("/");
                }
                _ => {
                    current_path = current_path.join(dir);
                    let parent = current_dir as *mut _;
                    current_dir =
                        current_dir
                            .children
                            .entry(dir.to_string())
                            .or_insert_with(|| {
                                let mut res = Dir::default();
                                res.parent = Some(parent);
                                res
                            });
                }
            }
        } else if line.starts_with("$ ls") {
            cmd = Cmd::Ls;
        } else {
            match cmd {
                Cmd::Cd => unreachable!(),
                Cmd::Ls => {
                    let mut s = line.split(" ");
                    let Ok::<i32, _>(size) = s.next().unwrap().parse() else {
                        continue;
                    };
                    let name = s.next().unwrap();

                    current_dir.files.insert(name.to_string(), size);
                }
            }
        }
    }
    root
}

fn part1(root: &Dir) -> i32 {
    let mut res = 0;
    for dir in root.iter_dirs() {
        let s = dir.size();
        if s < 100_000 {
            res += s;
        }
    }
    res
}

fn part2(root: &Dir) -> i32 {
    const TOTAL: i32 = 70000000;
    const REQUIRED: i32 = TOTAL - 30000000;
    let total_used = root.size();

    let mut result = total_used;
    for dir in root.iter_dirs() {
        let size = dir.size();
        if total_used - size < REQUIRED && size < result {
            result = size;
        }
    }
    result
}

fn main() {
    let inp = std::fs::read_to_string("input.txt").unwrap();
    let root = parse_input(&inp);
    let p1 = part1(&root);
    println!("part1: {p1}");
    let p2 = part2(&root);
    println!("part2: {p2}");
}

#[test]
fn part1_test() {
    let root = parse_input(
        r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#,
    );
    let res = part1(&root);

    assert_eq!(95437, res);
}
#[test]
fn part2_test() {
    let root = parse_input(
        r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#,
    );
    let res = part2(&root);

    assert_eq!(24933642, res);
}
