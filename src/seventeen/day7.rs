use fnv::FnvHashMap;

use failure::*;

use super::Result;
use parsing::day7;

type Name<'a> = &'a str;
type Attr<'a> = (u32, Vec<&'a str>);

pub struct Tree<'a> {
    pub root: Name<'a>,
    tree: FnvHashMap<Name<'a>, Attr<'a>>,
}

impl<'a> Tree<'a> {
    pub fn from_str(s: &'a str) -> Result<Self> {
        let tree: FnvHashMap<Name, Attr> = s.trim()
            .lines()
            .map(|l| {
                day7::line(l.as_bytes())
                    .to_result()
                    .map(|(n, w, c)| (n, (w, c)))
                    .map_err(Into::into)
            })
            .collect::<Result<_>>()?;

        ensure!(
            tree.iter()
                .all(|(_, (_, c))| c.iter().all(|n| tree.contains_key(n))),
            "child missing from tree"
        );

        let root = tree.iter()
            .find(|(n, _)| tree.iter().all(|(_, &(_, ref c))| !c.contains(n)))
            .ok_or_else(|| err_msg("unable to find root in tree"))
            .map(|(&n, _)| n)?;

        Ok(Tree { root, tree })
    }

    fn tree_weight(&self, root: &str) -> u32 {
        let (w, ref children) = self.tree[root];
        children.iter().fold(w, |acc, c| acc + self.tree_weight(c))
    }

    fn fix_tree(&self, offset: Option<i32>, root: &str) -> Option<u32> {
        let (w, ref children) = self.tree[root];
        let weights: Vec<u32> = children.iter().map(|c| self.tree_weight(c)).collect();
        let first = weights.first().unwrap();
        let balanced = weights.iter().all(|w| w == first);

        if balanced {
            offset.map(|v| (w as i32 + v) as u32)
        } else {
            let (i, off) = weights
                .iter()
                .enumerate()
                .find(|&(i, w)| weights.iter().enumerate().all(|(j, ww)| i == j || w != ww))
                .expect("wrong child weight not found");

            let normal = weights
                .iter()
                .find(|&w2| off != w2)
                .expect("normal child weight not found");

            let offset = *normal as i32 - *off as i32;
            self.fix_tree(Some(offset), children[i])
        }
    }

    pub fn solve(&self) -> u32 {
        self.fix_tree(None, self.root)
            .expect("No defect found in tree")
    }
}

pub fn solve() -> Result<()> {
    let input = super::get_input()?;
    let tree = Tree::from_str(&input)?;
    let first = tree.root;
    let second = tree.solve();

    println!(
        "Day 7:\n\
         Part 1: {}\n\
         Part 2: {}\n",
        first, second
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const IN: &str = "pbga (66)\n\
                      xhth (57)\n\
                      ebii (61)\n\
                      havc (66)\n\
                      ktlj (57)\n\
                      fwft (72) -> ktlj, cntj, xhth\n\
                      qoyq (66)\n\
                      padx (45) -> pbga, havc, qoyq\n\
                      tknk (41) -> ugml, padx, fwft\n\
                      jptl (61)\n\
                      ugml (68) -> gyxo, ebii, jptl\n\
                      gyxo (61)\n\
                      cntj (57)";

    #[test]
    fn test_rec_circus() {
        let tree = Tree::from_str(IN).unwrap();
        assert_eq!(tree.root, "tknk");
    }

    #[test]
    fn test_balance() {
        let tree = Tree::from_str(IN).unwrap();
        assert_eq!(tree.solve(), 60);
    }
}
