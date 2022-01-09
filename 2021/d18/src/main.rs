use std::fmt;
use std::fs::read_to_string;

fn content() -> Option<String> {
    read_to_string("./input.txt").ok()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum NodeVal {
    N(u32),
    P(usize),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Node {
    up: Option<usize>,
    left: Option<NodeVal>,
    right: Option<NodeVal>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Tree {
    tree: Vec<Node>,
    root: Option<usize>,
    dirty: bool,
}

impl Tree {
    fn new(input: &str) -> Option<Self> {
        let p: (Vec<Node>, Option<usize>, Option<usize>) = input
            .trim()
            .chars()
            .filter(|c| !c.is_whitespace())
            .fold((Vec::new(), None, None), |mut acc, c| {
                if c.is_digit(16) {
                    let d = c.to_digit(16).map(|d| NodeVal::N(d));
                    if acc.2.is_none() {
                        panic!("wrongly placed digit {}", c);
                    }
                    let current = acc.2.unwrap();
                    acc.0.get_mut(current).map({
                        |mut n| {
                            if n.left.is_none() {
                                n.left = d;
                            } else if n.right.is_none() {
                                n.right = d;
                            } else {
                                panic!("trying to add {} to a full node", c);
                            }
                        }
                    });
                    acc
                } else if c == '[' {
                    acc.0.push(Node {
                        up: acc.2,
                        left: None,
                        right: None,
                    });
                    let current = acc.0.len() - 1;
                    acc.2.map(|prev| {
                        acc.0.get_mut(prev).map(|prev| {
                            if prev.left.is_none() {
                                prev.left = Some(NodeVal::P(current));
                            } else if prev.right.is_none() {
                                prev.right = Some(NodeVal::P(current));
                            } else {
                                panic!("trying to add to a full node {:?}", prev);
                            }
                        });
                    });
                    let current = Some(current);
                    (acc.0, acc.1.or(current), current)
                } else if c == ']' {
                    if acc.2.is_none() {
                        panic!("trying to close an empty stack")
                    }
                    let current = acc.2.unwrap();
                    let prev = acc
                        .0
                        .get(current)
                        .map(|w| {
                            assert_eq!(w.up.is_none(), acc.1 == acc.2);
                            assert_eq!(w.left.is_some(), true);
                            assert_eq!(w.right.is_some(), true);
                            w.up
                        })
                        .flatten();
                    (acc.0, acc.1, prev)
                } else {
                    acc
                }
            });
        if p.0.is_empty() || p.1.is_none() {
            //panic!("Invalid input");
            return None;
        }
        p.1.map(|root| {
            p.0.get(root).map(|w| {
                assert_eq!(w.up.is_none(), true);
                assert_eq!(w.left.is_some(), true);
                assert_eq!(w.right.is_some(), true);
            })
        });

        Some(Tree {
            tree: p.0,
            root: p.1,
            dirty: true,
        })
    }

    fn lines(input: &str) -> Option<Self> {
        input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .fold(None, |acc, line| match acc {
                Some(prev) => Tree::new(line).as_ref().map(|next| prev.add(next).reduce()),
                None => Tree::new(line),
            })
    }

    fn fmt(&self, node: usize, f: &mut fmt::Formatter) -> fmt::Result {
        let node = self.tree.get(node).unwrap();

        write!(f, "[")
            .and(match node.left {
                Some(NodeVal::P(l)) => self.fmt(l, f),
                Some(NodeVal::N(n)) => write!(f, "{}", n),
                None => panic!("Invalid node"),
            })
            .and(write!(f, ","))
            .and(match node.right {
                Some(NodeVal::P(r)) => self.fmt(r, f),
                Some(NodeVal::N(n)) => write!(f, "{}", n),
                None => panic!("Invalid node"),
            })
            .and(write!(f, "]"))
    }

    // Find the first node if any which fulfills a criteria
    fn walk<F: Fn(&Node, usize) -> bool + Copy>(
        &self,
        node: usize,
        depth: usize,
        criteria: F,
    ) -> Option<usize> {
        self.tree.get(node).map_or(None, |n| {
            match n.left {
                Some(NodeVal::P(p)) => self.walk(p, depth + 1, criteria),
                _ => None,
            }
            .or(if criteria(n, depth) { Some(node) } else { None })
            .or(match n.right {
                Some(NodeVal::P(p)) => self.walk(p, depth + 1, criteria),
                _ => None,
            })
        })
    }

    pub fn split(&self) -> Self {
        let mut c = self.clone();
        let crit = |node: &Node, _depth: usize| -> bool {
            let l = match node.left {
                Some(NodeVal::N(n)) => n > 9,
                _ => false,
            };
            let r = match node.right {
                Some(NodeVal::N(n)) => n > 9,
                _ => false,
            };
            l || r
        };
        match c.walk(c.root.unwrap(), 0, crit) {
            Some(pos) => {
                let s_node = c.tree.get(pos).unwrap();
                let cont = match s_node.left {
                    Some(NodeVal::N(val)) => {
                        if val > 9 {
                            let v1 = val / 2;
                            let v2 = (val + 1) / 2;
                            c.tree.push(Node {
                                up: Some(pos),
                                left: Some(NodeVal::N(v1)),
                                right: Some(NodeVal::N(v2)),
                            });
                            let current = c.tree.len() - 1;
                            c.tree
                                .get_mut(pos)
                                .map(|node| node.left = Some(NodeVal::P(current)));
                            c.dirty = true;
                            false
                        } else {
                            true
                        }
                    }
                    _ => true,
                };
                if cont {
                    let s_node = c.tree.get(pos).unwrap();
                    match s_node.right {
                        Some(NodeVal::N(val)) => {
                            if val > 9 {
                                let v1 = val / 2;
                                let v2 = (val + 1) / 2;
                                c.tree.push(Node {
                                    up: Some(pos),
                                    left: Some(NodeVal::N(v1)),
                                    right: Some(NodeVal::N(v2)),
                                });
                                let current = c.tree.len() - 1;
                                c.tree
                                    .get_mut(pos)
                                    .map(|node| node.right = Some(NodeVal::P(current)));
                                c.dirty = true;
                            }
                        }
                        _ => (),
                    }
                }
            }
            None => (),
        };

        c
    }

    pub fn explode(&self) -> Self {
        let mut c = self.clone();
        let crit = |node: &Node, depth: usize| -> bool {
            match node.left_num() {
                Some(_) => match node.right_num() {
                    Some(_) => depth > 3,
                    _ => false,
                },
                _ => false,
            }
        };
        loop {
            match c.walk(c.root.unwrap(), 0, crit) {
                Some(pos) => {
                    let node = *c.tree.get(pos).unwrap();
                    let left_val = node.left_num().unwrap();
                    let right_val = node.right_num().unwrap();

                    {
                        let mut from = pos;
                        let mut p = node.up;

                        while p.is_some() {
                            let pup = p.unwrap();

                            let cont = c
                                .tree
                                .get_mut(pup)
                                .map(|u| match u.right {
                                    Some(NodeVal::P(nl)) => {
                                        if nl == from {
                                            match u.left_num() {
                                                Some(n) => {
                                                    u.left = Some(NodeVal::N(n + left_val));
                                                    (false, None)
                                                }
                                                _ => (false, p),
                                            }
                                        } else {
                                            from = pup;
                                            (true, u.up)
                                        }
                                    }
                                    _ => {
                                        from = pup;
                                        (true, u.up)
                                    }
                                })
                                .unwrap();
                            p = cont.1;
                            if !cont.0 {
                                break;
                            }
                        }

                        if p.is_some() {
                            p = c
                                .tree
                                .get(p.unwrap())
                                .map(|root| match root.left {
                                    Some(NodeVal::P(n)) => Some(n),
                                    _ => None,
                                })
                                .flatten();
                            while p.is_some() {
                                let cont = c.tree.get_mut(p.unwrap()).map(|u| match u.right {
                                    Some(NodeVal::N(n)) => {
                                        u.right = Some(NodeVal::N(n + left_val));
                                        (false, None)
                                    }
                                    Some(NodeVal::P(nl)) => (true, Some(nl)),
                                    None => unreachable!("never"),
                                });
                                match cont {
                                    Some((false, _)) => break,
                                    Some((true, down)) => p = down,
                                    None => unreachable!("never"),
                                }
                            }
                        }
                    }

                    {
                        let mut from = pos;
                        let mut p = node.up;

                        while p.is_some() {
                            let pup = p.unwrap();
                            let cont = c
                                .tree
                                .get_mut(pup)
                                .map(|u| match u.left {
                                    Some(NodeVal::P(nl)) => {
                                        if nl == from {
                                            match u.right_num() {
                                                Some(n) => {
                                                    u.right = Some(NodeVal::N(n + right_val));
                                                    (false, None)
                                                }
                                                _ => (false, p),
                                            }
                                        } else {
                                            from = pup;
                                            (true, u.up)
                                        }
                                    }
                                    _ => {
                                        from = pup;
                                        (true, u.up)
                                    }
                                })
                                .unwrap();
                            p = cont.1;
                            if !cont.0 {
                                break;
                            }
                        }

                        if p.is_some() {
                            p = c
                                .tree
                                .get(p.unwrap())
                                .map(|root| match root.right {
                                    Some(NodeVal::P(n)) => Some(n),
                                    _ => None,
                                })
                                .flatten();
                            while p.is_some() {
                                let cont = c.tree.get_mut(p.unwrap()).map(|u| match u.left {
                                    Some(NodeVal::N(n)) => {
                                        u.left = Some(NodeVal::N(n + right_val));
                                        (false, None)
                                    }
                                    Some(NodeVal::P(nl)) => (true, Some(nl)),
                                    None => unreachable!("never"),
                                });
                                match cont {
                                    Some((false, _)) => break,
                                    Some((true, down)) => p = down,
                                    None => unreachable!("never"),
                                }
                            }
                        }
                    }

                    c.tree.get_mut(node.up.unwrap()).map(|u| {
                        if u.left == Some(NodeVal::P(pos)) {
                            u.left = Some(NodeVal::N(0));
                        } else if u.right == Some(NodeVal::P(pos)) {
                            u.right = Some(NodeVal::N(0));
                        }
                    });
                }
                None => break,
            }
        }
        c.dirty = false;
        c
    }

    pub fn reduce(&self) -> Self {
        let mut w = self.clone();
        w.dirty = true;

        while w.dirty {
            w = w.explode().split();
        }
        w
    }

    pub fn add(&self, other: &Self) -> Self {
        let mut w = self.clone();
        w.dirty = true;
        fn f(
            into: &mut Vec<Node>,
            other: &Vec<Node>,
            n: usize,
            up: Option<usize>,
        ) -> Option<usize> {
            other.get(n).map(|node| {
                into.push(Node {
                    up: up,
                    left: None,
                    right: None,
                });
                let current = into.len() - 1;
                let l = match node.left {
                    Some(NodeVal::N(_num)) => node.left,
                    Some(NodeVal::P(lp)) => {
                        let s = f(into, other, lp, Some(current));
                        Some(NodeVal::P(s.unwrap()))
                    }
                    None => panic!("nodes should always have values"),
                };
                let r = match node.right {
                    Some(NodeVal::N(_num)) => node.right,
                    Some(NodeVal::P(rp)) => {
                        let s = f(into, other, rp, Some(current));
                        Some(NodeVal::P(s.unwrap()))
                    }
                    None => panic!("nodes should always have values"),
                };
                into.get_mut(current).map(|c| {
                    c.left = l;
                    c.right = r;
                });
                current
            })
        }
        let sub = f(&mut w.tree, &other.tree, other.root.unwrap(), None);
        w.tree.push(Node {
            up: None,
            left: w.root.map(|l| NodeVal::P(l)),
            right: sub.map(|r| NodeVal::P(r)),
        });
        let new_root = w.tree.len() - 1;
        w.tree.get_mut(w.root.unwrap()).map(|node| {
            node.up = Some(new_root);
        });
        w.tree.get_mut(sub.unwrap()).map(|node| {
            node.up = Some(new_root);
        });
        w.root = Some(new_root);
        w
    }

    pub fn magnitude(&self) -> usize {
        self.root.map_or(0, |root| self.magnitude_i(root))
    }

    fn magnitude_i(&self, node: usize) -> usize {
        let node = self.tree.get(node).unwrap();

        let l = match node.left {
            Some(NodeVal::P(l)) => self.magnitude_i(l),
            Some(NodeVal::N(n)) => n as usize,
            None => 0,
        };
        let r = match node.right {
            Some(NodeVal::P(r)) => self.magnitude_i(r),
            Some(NodeVal::N(n)) => n as usize,
            None => 0,
        };
        3 * l + 2 * r
    }
}

impl Node {
    pub fn left_num(self) -> Option<u32> {
        match self.left {
            Some(NodeVal::N(n)) => Some(n),
            _ => None,
        }
    }

    pub fn right_num(self) -> Option<u32> {
        match self.right {
            Some(NodeVal::N(n)) => Some(n),
            _ => None,
        }
    }
}
impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.root.map_or(write!(f, ""), |root| self.fmt(root, f))
    }
}

fn solution_a(input: &str) -> Option<usize> {
    Tree::lines(input).map(|t| {
        t.magnitude()
    })
}

fn solution_b(input: &str) -> Option<usize> {
    let trees: Vec<Tree> = input
        .lines()
        .map(|line| {
            let line = line.trim();
            let p = Tree::new(line.trim());
            p
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    let mut maxi = None;
    for i in 0..trees.len() {
        for j in 0..trees.len() {
            if i != j {
                trees.get(i).map(|x| {
                    trees.get(j).map(|y| {
                        let xy = x.add(y).reduce();
                        let m = xy.magnitude();
                        maxi = Some(maxi.map_or(m, |p| if m > p { m } else { p }));
                    })
                });
            }
        }
    }
    maxi
}

fn main() {
    let c = content().unwrap();

    let a = solution_a(&c);
    let b = solution_b(&c);

    println!("Step A: {:?}", a);
    println!("Step B: {:?}", b);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simple() -> Option<String> {
        read_to_string("./simple.txt").ok()
    }

    #[test]
    fn test_file_reading() {
        assert_ne!(content().is_none(), true);
    }

    #[test]
    fn test_reader() {
        let data = simple().unwrap();
        data.lines().for_each(|line| {
            let line = line.trim();
            let p = Tree::new(line).unwrap();
            assert_eq!(line, p.to_string());
        });
    }

    fn test_explode_ab(a: &str, b: &str) {
        let p = Tree::new(a).map(|p| p.reduce()).unwrap();
        assert_eq!(p.to_string(), b);
    }

    #[test]
    fn test_explode() {
        test_explode_ab("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        test_explode_ab("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        test_explode_ab("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        test_explode_ab(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
        test_explode_ab(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
    }

    #[test]
    fn test_split() {
        test_explode_ab("[a,b]", "[[5,5],[5,6]]");
        test_explode_ab("[a,9]", "[[5,5],9]");
        test_explode_ab("[9,b]", "[9,[5,6]]");
    }

    #[test]
    fn test_add() {
        let a = "[[1,2],[3,4]]";
        let b = "[[5,6],[7,8]]";
        let aa = Tree::new(a).unwrap();
        let bb = Tree::new(b).unwrap();

        assert_eq!(aa.add(&bb).to_string(), "[[[1,2],[3,4]],[[5,6],[7,8]]]");
    }

    #[test]
    fn test_reduce() {
        let a = "[[[[4,3],4],4],[7,[[8,4],9]]]";
        let b = "[1,1]";
        let c = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
        let aa = Tree::new(a).unwrap();
        let bb = Tree::new(b).unwrap();

        assert_eq!(aa.add(&bb).reduce().to_string(), c);
    }

    #[test]
    fn test_l4() {
        let a = "[1,1]
        [2,2]
        [3,3]
        [4,4]";
        let b = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
        assert_eq!(Tree::lines(a).unwrap().to_string(), b);
    }

    #[test]
    fn test_l5() {
        let a = "[1,1]
        [2,2]
        [3,3]
        [4,4]
        [5,5]";
        let b = "[[[[3,0],[5,3]],[4,4]],[5,5]]";
        assert_eq!(Tree::lines(a).unwrap().reduce().to_string(), b);
    }

    #[test]
    fn test_l6() {
        let a = "[1,1]
        [2,2]
        [3,3]
        [4,4]
        [5,5]
        [6,6]
        ";
        let b = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
        assert_eq!(Tree::lines(a).unwrap().to_string(), b);
    }

    #[test]
    fn test_lines() {
        let a = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        [7,[5,[[3,8],[1,4]]]]
        [[2,[2,2]],[8,[8,1]]]
        [2,9]
        [1,[[[9,3],9],[[9,0],[0,7]]]]
        [[[5,[7,4]],7],1]
        [[[[4,2],2],6],[8,7]]";
        let b = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";
        assert_eq!(Tree::lines(a).unwrap().to_string(), b);
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Tree::new("[9,1]").map(|p| p.magnitude()), Some(29));
        assert_eq!(Tree::new("[1,9]").map(|p| p.magnitude()), Some(21));
        assert_eq!(Tree::new("[[9,1],[1,9]]").map(|p| p.magnitude()), Some(129));
        assert_eq!(
            Tree::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .map(|p| p.magnitude()),
            Some(3488)
        );
    }

    #[test]
    fn test_simple_a() {
        let data = simple().unwrap();
        assert_eq!(solution_a(&data), Some(4140));
    }

    #[test]
    fn test_simple_b() {
        let data = simple().unwrap();
        assert_eq!(solution_b(&data), Some(3993));
    }

    #[test]
    fn test_solution_a() {
        let c = content().unwrap();
        assert_eq!(solution_a(&c), Some(4176));
    }

    #[test]
    fn test_solution_b() {
        let c = content().unwrap();
        assert_eq!(solution_b(&c), Some(4633));
    }
}
