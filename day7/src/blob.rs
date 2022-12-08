use std::collections::HashMap;

pub enum Blob {
    File(String, usize),
    Dir(String, Vec<Blob>),
}

// recursive
fn get_size(blob: Blob, map: &mut HashMap<String, usize>) -> usize {
    match blob {
        Blob::File(name, size) => {
            dbg!(name, size);
            size
        }
        Blob::Dir(name, children) => {
            dbg!(&name);
            let mut size = 0;
            for child in children {
                size += get_size(child, map);
            }
            map.insert(name, size);
            size
        }
    }
}

// return an iterator?
// gotta go through the whole thing first to determine all sizes

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursing_blob() {
        // step 1 need to parse into this data structure
        let mut sizes = HashMap::new();
        let root = Blob::Dir(
            "/".into(),
            vec![
                Blob::Dir(
                    "a".into(),
                    vec![
                        Blob::Dir("e".into(), vec![Blob::File("i".into(), 584)]),
                        Blob::File("f".into(), 29116),
                        Blob::File("g".into(), 2557),
                        Blob::File("h.lst".into(), 62596),
                    ],
                ),
                Blob::File("b.txt".into(), 14848514),
                Blob::File("c.dat".into(), 8504156),
                Blob::Dir(
                    "d".into(),
                    vec![
                        Blob::File("j.txt".into(), 4060174),
                        Blob::File("d.log".into(), 8033020),
                        Blob::File("d.ext".into(), 5626152),
                        Blob::File("k".into(), 7214296),
                    ],
                ),
            ],
        );

        get_size(root, &mut sizes);
        dbg!(&sizes);
        let exp = HashMap::from([
            ("e".into(), 584),
            ("a".into(), 94853),
            ("d".into(), 24933642),
            ("/".into(), 48381165),
        ]);
        let answer: usize = sizes.values().filter(|&&x| x <= 100000).sum();
        let exp_ans = 95437;
        assert_eq!(sizes, exp);
        assert_eq!(answer, exp_ans);
    }
}
