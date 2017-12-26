use std::str;
use std::collections::HashMap;

use failure::*;

type Program<'a> = (&'a str, Attr<'a>);
type Attr<'a> = (u32, Vec<&'a str>);

fn tree_weight(tree: &HashMap<&str, Attr>, root: &str) -> u32 {
    let &(w, ref children) = tree.get(root).unwrap();
    let mut result = w;
    for c in children {
        result += tree_weight(tree, c);
    }

    result
}

fn fix_imbalance(
    tree: &HashMap<&str, (u32, Vec<&str>)>,
    root: &str,
    mut offset: i32,
) -> Option<u32> {
    let &(w, ref children) = tree.get(root).unwrap();
    let mut result = None;
    if offset == 0 {
        for (i, val) in children.iter().map(|v| tree_weight(tree, v)).enumerate() {
            if children
                .iter()
                .enumerate()
                .filter(|&(j, _)| j != i)
                .map(|(_, v)| tree_weight(tree, v))
                .all(|v| val != v)
            {
                if let Some((_, v)) = children
                    .iter()
                    .map(|v| tree_weight(tree, v))
                    .enumerate()
                    .find(|&(j, _)| j != i)
                {
                    offset = v as i32 - val as i32;
                    result = fix_imbalance(tree, &children[i], offset);
                    break;
                }
            }
        }
        result
    } else {
        let mut it = children.iter().map(|v| tree_weight(tree, v));
        let first = it.next().unwrap();
        if it.all(|v| v == first) {
            Some((w as i32 + offset) as u32)
        } else {
            for (i, val) in children.iter().map(|v| tree_weight(tree, v)).enumerate() {
                if children
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, v)| tree_weight(tree, v))
                    .all(|v| val != v)
                {
                    if let Some((_, v)) = children
                        .iter()
                        .map(|v| tree_weight(tree, v))
                        .enumerate()
                        .find(|&(j, _)| j != i)
                    {
                        offset = v as i32 - val as i32;
                        result = fix_imbalance(tree, &children[i], offset);
                        break;
                    }
                }
            }
            result
        }
    }
}

pub fn rec_circus(input: &str) -> &str {
    let input = input.lines().map(|s| {
        let mut it = s.split_whitespace();
        let name = it.next().unwrap();
        let num = it.next().unwrap();
        let children = if it.next().is_some() {
            let result = it.map(|s| s.replace(",", "")).collect::<Vec<String>>();
            Some(result)
        } else {
            None
        };
        (name, num, children)
    });

    let mut seen = HashMap::new();
    let mut root = "";

    for (name, _, children) in input {
        if let Some(ch) = children {
            for s in ch {
                root = name;
                seen.insert(s, name);
            }
        }
    }

    while seen.contains_key(root) {
        root = seen[root];
    }

    root
}

pub fn balance(input: &str) -> Result<u32, Error> {
    let brackets: &[char] = &['(', ')'];
    let input: Vec<Program> = input
        .lines()
        .map(|s| {
            let mut it = s.split("->").map(|s| s.trim());
            let mut attr = it.next().unwrap().split_whitespace();
            let key = attr.next().unwrap();
            let w: u32 = attr.next().unwrap().trim_matches(brackets).parse()?;

            let children: Vec<&str> = match it.next() {
                Some(s) => s.split(", ").collect(),
                None => vec![],
            };

            Ok((key, (w, children)))
        })
        .collect::<Result<_, Error>>()?;

    let mut tree = HashMap::new();
    let mut parents = HashMap::new();
    let mut root = "";

    for (key, (w, children)) in input {
        for c in children.iter() {
            root = key;
            parents.insert(*c, key);
        }

        tree.insert(key, (w, children));
    }

    while parents.contains_key(root) {
        root = parents[root];
    }

    Ok(fix_imbalance(&tree, root, 0).unwrap())
}

#[cfg(test)]
mod tests {
    use seventeen::check;
    use super::*;
    const IN: &str = "pbga (66)\nxhth (57)\nebii (61)\nhavc (66)\nktlj (57)\nfwft (72) -> ktlj, cntj, xhth\nqoyq (66)\npadx (45) -> pbga, havc, qoyq\ntknk (41) -> ugml, padx, fwft\njptl (61)\nugml (68) -> gyxo, ebii, jptl\ngyxo (61)\ncntj (57)";

    #[test]
    fn test_rec_circus() {
        assert_eq!(rec_circus(IN), "tknk");
    }

    #[test]
    fn test_balance() {
        check(balance(IN), 60);
    }
}
