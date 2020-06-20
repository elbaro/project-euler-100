
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let mut s: HashSet<u32> = HashSet::new();
    let paths = std::fs::read_dir("src/bin").unwrap();
    for path in paths {
        let id = path
            .unwrap()
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        if let Ok(id) = id.parse::<u32>() {
            s.insert(id);
        }
    }

    let mx = *s.iter().max().unwrap();

    &(1u32..mx+1).chunks(10).into_iter().for_each(| x| {
        println!("| {} |", x.map(|id| {
            let path = format!("src/bin/{}.rs", id);
            if std::path::Path::new(&path).exists() {
                format!("[{id}](https://github.com/elbaro/project-euler/blob/master/src/bin/{id}.rs)", id = id)
            } else {
                "".to_string()
            }
        }).join(" | "));
    });
}
