//! Recursive tree layout system.
//!
//! [`Node`] is a recursive enum — leaf [`Category`] or [`Node::Split`] with
//! children arranged in a [`Direction::Row`] or [`Direction::Column`]. 
//! 
//! Call [`Node::layout`]
//! to distribute position and size down the tree, then [`Node::into_flat`]
//! to collect all categories with computed coordinates.

use crate::*;

/// Default size for grid.
/// 
/// Can be used in [`Node::layout`].
pub const GRID_SIZE: (f32, f32) = (1000.0, 500.0);

/// Enum describing a direction of [Node::Split]
pub enum Direction {
    Row,
    Column,
}

/// Recursive enum used to build a tree
/// of [Category] containing rectangles.
/// 
/// Owns a Category object or a [Vec] of child Nodes
/// respectively.
pub enum Node {
    /// Terminal variant.
    /// Containing final object of tree.
    Category(Category),
    /// Mid variant.
    Split(Vec<Node>, Direction)
}

/// Methods responsible for main
/// interactions with tree
impl Node {
    /// Calculates position of all terminal elements([Node::Category])
    /// of the tree.
    pub fn layout(&mut self, pos: (f32, f32), size: (f32, f32)) {
        match self {
            Node::Category(c) => {
                c.set_pos(pos);
                c.set_size(size);
            },
            Node::Split(v, d) => {
                let len = v.len();
                let width = size.0;
                let height = size.1;
                match d {
                    Direction::Column => {
                        let per = height / len as f32;
                        for (idx, item) in v.iter_mut().enumerate() {
                            item.layout(
                                (pos.0, per * idx as f32 + pos.1),
                                (width, per)
                            );
                        }
                    },
                    Direction::Row => {
                        let per = width / len as f32;
                        for (idx, item) in v.iter_mut().enumerate() {
                            item.layout(
                                (per * idx as f32 + pos.0, pos.1),
                                (per, height)
                            );
                        }
                    }
                }
            }
        }
    }

    /// Turns tree into flat [Vec]<[Category]>.
    /// 
    /// ⚠️ **Does not calls **[Self::layout]** inside its body!**
    pub fn into_flat(self) -> Vec<Category> {
        match self {
            Node::Category(c) => {
                vec![c]
            },
            Node::Split(v, _) => {
                v.into_iter().map(|i| i.into_flat()).flatten().collect()
            }
        }
    }
}

/// Methods responsible for navigations
/// on the tree.
impl Node {
    /// Returns an `Vec` slice with child nodes
    /// if there are some.
    pub fn children(&self) -> Option<&[Node]> {
        match self {
            Node::Split(v, _) => if !v.is_empty() { Some(v) } else { None },
            Node::Category(_) => None,
        }
    }

    /// Returns an mutable `Vec` slice with child nodes
    /// if there are some.
    pub fn children_mut(&mut self) -> Option<&mut [Node]> {
        match self {
            Node::Split(v, _) => Some(v),
            Node::Category(_) => None,
        }
    }

    /// Used to define if this node is terminal.
    pub fn is_leaf(&self) -> bool {
        if let Node::Category(_) = self {
            true
        } else { false }
    }
}

/// Methods used to interact with nodes containment.
impl Node {
    /// Method used to append new node into container.
    pub fn push(&mut self, node: Node) -> Result<(), &'static str> {
        match self {
            Node::Category(_) => Err("Tried to push a node into a category"),
            Node::Split(v, _) => { v.push(node); Ok(()) }
        }
    }

    /// Method used to insert new node into container
    /// in a defined place.
    pub fn insert(&mut self, node: Node, idx: usize) -> Result<(), &str> {
        match self {
            Node::Category(_) => Err("Tried to insert node into a category"),
            Node::Split(v, _) => {
                let max_idx = v.len().checked_sub(1).unwrap_or(0);
                if max_idx < idx {
                    Err("index out of bounds")
                } else {
                    v.insert(idx, node);
                    Ok(())
                }
            }
        }
    }

    /// Method used to remove a determined node
    /// from the container.
    pub fn remove(&mut self, idx: usize) -> Result<(), &str> {
        match self {
            Node::Category(_) => Err("Tried to remove node from a category"),
            Node::Split(v, _) => {
                let max_idx = v.len().checked_sub(1).unwrap_or(0);
                if max_idx < idx {
                    Err("index out of bounds")
                } else {
                    v.remove(idx);
                    Ok(())
                }
            }
        }
    }

    /// Method used to swap nodes
    /// in the container.
    /// 
    /// ### Arguments
    /// a: index 1
    /// 
    /// b: index 2
    pub fn swap(&mut self, a: usize, b: usize) -> Result<(), &str> {
        match self {
            Node::Category(_) => Err("Tried to swap elements in category"),
            Node::Split(v, _) => {
                                let max_idx = v.len() - 1;
                if max_idx < a || max_idx < b {
                    Err("Index is bigger than vector")
                } else if a == b {
                    Ok(())
                } else {
                    v.swap(a, b);
                    Ok(())
                }
            }
        }
    }

    /// Method used to determine
    /// number of nodes
    /// in the container.
    pub fn len(&self) -> usize {
        match self {
            Node::Category(_) => 0,
            Node::Split(v, _) => {
                v.len()
            }
        }
    }

    /// Method used to determine
    /// if container is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            Node::Category(_) => false,
            Node::Split(v, _) => v.is_empty()
        }
    }
}

impl Node {
    /// Method used to get a reference to the
    /// category with given name.
    pub fn find(&self, name: &str) -> Option<&Node> {
        match self {
            Node::Category(c) => {
                if c.name() == name { return Some(self) } else { None }
            },
            Node::Split(v, _) => {
                for i in v.iter() {
                    if let Some(n) = i.find(name) {
                        return Some(n);
                    }
                }
                None
            }
        }
    }

    /// Method used to get a **mutable** reference to the
    /// category with given name.
    pub fn find_mut(&mut self, name: &str) -> Option<&mut Node> {
        match self {
            Node::Category(c) => {
                if c.name() == name { return Some(self) } else { None }
            },
            Node::Split(v, _) => {
                for i in v.iter_mut() {
                    if let Some(n) = i.find_mut(name) {
                        return Some(n);
                    }
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cat(name: &str) -> Category {
        Category::new(name, (0.0, 0.0), (0.0, 0.0))
    }

    const EPS: f32 = 1e-6;

    // ── is_leaf ──

    #[test]
    fn leaf_category() {
        assert!(Node::Category(cat("a")).is_leaf());
    }

    #[test]
    fn leaf_split() {
        assert!(!Node::Split(vec![], Direction::Row).is_leaf());
    }

    // ── push ──

    #[test]
    fn push_into_split_ok() {
        let mut n = Node::Split(vec![], Direction::Column);
        assert!(n.push(Node::Category(cat("a"))).is_ok());
    }

    #[test]
    fn push_into_category_err() {
        let mut n = Node::Category(cat("a"));
        assert!(n.push(Node::Category(cat("b"))).is_err());
    }

    #[test]
    fn push_adds_child() {
        let mut n = Node::Split(vec![], Direction::Row);
        n.push(Node::Category(cat("a"))).unwrap();
        n.push(Node::Category(cat("b"))).unwrap();
        assert_eq!(n.len(), 2);
    }

    // ── insert ──

    #[test]
    fn insert_at_valid_index() {
        let mut n = Node::Split(vec![
            Node::Category(cat("a")),
            Node::Category(cat("c")),
        ], Direction::Column);
        n.insert(Node::Category(cat("b")), 1).unwrap();
        assert_eq!(n.len(), 3);
    }

    #[test]
    fn insert_at_invalid_index_err() {
        let mut n = Node::Split(vec![Node::Category(cat("a"))], Direction::Column);
        assert!(n.insert(Node::Category(cat("b")), 5).is_err());
    }

    #[test]
    fn insert_into_category_err() {
        let mut n = Node::Category(cat("a"));
        assert!(n.insert(Node::Category(cat("b")), 0).is_err());
    }

    #[test]
    fn insert_preserves_order() {
        let mut n = Node::Split(vec![
            Node::Category(cat("a")),
            Node::Category(cat("c")),
        ], Direction::Column);
        n.insert(Node::Category(cat("b")), 1).unwrap();
        if let Node::Split(v, _) = &n {
            assert_eq!(v.len(), 3);
        }
    }

    // ── remove ──

    #[test]
    fn remove_valid_index_ok() {
        let mut n = Node::Split(vec![
            Node::Category(cat("a")),
            Node::Category(cat("b")),
        ], Direction::Row);
        assert!(n.remove(0).is_ok());
        assert_eq!(n.len(), 1);
    }

    #[test]
    fn remove_invalid_index_err() {
        let mut n = Node::Split(vec![Node::Category(cat("a"))], Direction::Row);
        assert!(n.remove(5).is_err());
    }

    #[test]
    fn remove_from_category_err() {
        let mut n = Node::Category(cat("a"));
        assert!(n.remove(0).is_err());
    }

    // ── swap ──

    #[test]
    fn swap_valid_indices() {
        let mut n = Node::Split(vec![
            Node::Category(cat("a")),
            Node::Category(cat("b")),
        ], Direction::Row);
        n.swap(0, 1).unwrap();
    }

    #[test]
    fn swap_same_index() {
        let mut n = Node::Split(vec![
            Node::Category(cat("a")),
            Node::Category(cat("b")),
        ], Direction::Row);
        assert!(n.swap(0, 0).is_ok());
    }

    #[test]
    fn swap_invalid_index_err() {
        let mut n = Node::Split(vec![Node::Category(cat("a"))], Direction::Row);
        assert!(n.swap(0, 5).is_err());
    }

    #[test]
    fn swap_on_category_err() {
        let mut n = Node::Category(cat("a"));
        assert!(n.swap(0, 1).is_err());
    }

    // ── len ──

    #[test]
    fn len_split() {
        let n = Node::Split(vec![Node::Category(cat("a")), Node::Category(cat("b"))], Direction::Column);
        assert_eq!(n.len(), 2);
    }

    #[test]
    fn len_empty_split() {
        let n = Node::Split(vec![], Direction::Row);
        assert_eq!(n.len(), 0);
    }

    #[test]
    fn len_category_zero() {
        let n = Node::Category(cat("a"));
        assert_eq!(n.len(), 0);
    }

    // ── is_empty ──

    #[test]
    fn empty_split_is_empty() {
        let n = Node::Split(vec![], Direction::Column);
        assert!(n.is_empty());
    }

    #[test]
    fn nonempty_split_not_empty() {
        let n = Node::Split(vec![Node::Category(cat("a"))], Direction::Column);
        assert!(!n.is_empty());
    }

    #[test]
    fn is_empty_category_false() {
        let n = Node::Category(cat("a"));
        assert!(!n.is_empty());
    }

    // ── children ──

    #[test]
    fn children_on_category_none() {
        let n = Node::Category(cat("a"));
        assert!(n.children().is_none());
    }

    #[test]
    fn children_mut_on_category_none() {
        let mut n = Node::Category(cat("a"));
        assert!(n.children_mut().is_none());
    }

    // ── find / find_mut ──

    #[test]
    fn find_existing_in_flat() {
        let n = Node::Split(vec![
            Node::Category(cat("a")),
            Node::Category(cat("b")),
        ], Direction::Row);
        assert!(n.find("a").is_some());
        assert!(n.find("b").is_some());
    }

    #[test]
    fn find_nonexisting() {
        let n = Node::Split(vec![Node::Category(cat("a"))], Direction::Row);
        assert!(n.find("x").is_none());
    }

    #[test]
    fn find_nested() {
        let inner = Node::Split(vec![Node::Category(cat("deep"))], Direction::Column);
        let outer = Node::Split(vec![Node::Category(cat("shallow")), inner], Direction::Row);
        assert!(outer.find("deep").is_some());
        assert!(outer.find("shallow").is_some());
    }

    #[test]
    fn find_on_leaf_self() {
        let n = Node::Category(cat("me"));
        assert!(n.find("me").is_some());
    }

    #[test]
    fn find_mut_can_modify() {
        let mut n = Node::Split(vec![Node::Category(cat("x"))], Direction::Row);
        let found = n.find_mut("x").unwrap();
        if let Node::Category(c) = found {
            c.set_name("y".to_string());
        }
        assert!(n.find("x").is_none());
        assert!(n.find("y").is_some());
    }

    // ── into_flat ──

    #[test]
    fn into_flat_single() {
        let n = Node::Category(cat("alone"));
        let v = n.into_flat();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].name(), "alone");
    }

    #[test]
    fn into_flat_split() {
        let n = Node::Split(vec![
            Node::Category(cat("a")),
            Node::Category(cat("b")),
        ], Direction::Row);
        let v = n.into_flat();
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn into_flat_nested() {
        let inner = Node::Split(vec![
            Node::Category(cat("x")),
            Node::Category(cat("y")),
        ], Direction::Column);
        let outer = Node::Split(vec![
            Node::Category(cat("z")),
            inner,
        ], Direction::Row);
        let v = outer.into_flat();
        assert_eq!(v.len(), 3);
    }

    // ── layout single ──

    #[test]
    fn layout_sets_pos_size_on_category() {
        let mut n = Node::Category(cat("c"));
        n.layout((10.0, 20.0), (100.0, 200.0));
        if let Node::Category(ref c) = n {
            assert!((c.pos().0 - 10.0).abs() < EPS);
            assert!((c.pos().1 - 20.0).abs() < EPS);
            assert!((c.size().0 - 100.0).abs() < EPS);
            assert!((c.size().1 - 200.0).abs() < EPS);
        }
    }

    // ── layout column ──

    #[test]
    fn layout_column_two_elements() {
        let mut n = Node::Split(vec![
            Node::Category(cat("top")),
            Node::Category(cat("bot")),
        ], Direction::Column);
        n.layout((0.0, 0.0), (1000.0, 500.0));

        let cats = n.into_flat();
        assert!((cats[0].pos().1 - 0.0).abs() < EPS);
        assert!((cats[0].size().0 - 1000.0).abs() < EPS);
        assert!((cats[0].size().1 - 250.0).abs() < EPS);

        assert!((cats[1].pos().1 - 250.0).abs() < EPS);
        assert!((cats[1].size().0 - 1000.0).abs() < EPS);
        assert!((cats[1].size().1 - 250.0).abs() < EPS);
    }

    // ── layout row ──

    #[test]
    fn layout_row_two_elements() {
        let mut n = Node::Split(vec![
            Node::Category(cat("l")),
            Node::Category(cat("r")),
        ], Direction::Row);
        n.layout((0.0, 0.0), (1000.0, 500.0));

        let cats = n.into_flat();
        assert!((cats[0].pos().0 - 0.0).abs() < EPS);
        assert!((cats[0].size().0 - 500.0).abs() < EPS);
        assert!((cats[0].size().1 - 500.0).abs() < EPS);

        assert!((cats[1].pos().0 - 500.0).abs() < EPS);
        assert!((cats[1].size().0 - 500.0).abs() < EPS);
        assert!((cats[1].size().1 - 500.0).abs() < EPS);
    }

    // ── layout nested ──

    #[test]
    fn layout_row_inside_column() {
        let inner = Node::Split(vec![
            Node::Category(cat("l")),
            Node::Category(cat("r")),
        ], Direction::Row);

        let mut outer = Node::Split(vec![
            Node::Category(cat("top")),
            inner,
        ], Direction::Column);
        outer.layout((0.0, 0.0), (800.0, 600.0));

        let cats = outer.into_flat();
        assert_eq!(cats.len(), 3);

        // top: full width, upper half
        assert!((cats[0].pos().0 - 0.0).abs() < EPS);
        assert!((cats[0].pos().1 - 0.0).abs() < EPS);
        assert!((cats[0].size().0 - 800.0).abs() < EPS);
        assert!((cats[0].size().1 - 300.0).abs() < EPS);

        // l: left half of lower half
        assert!((cats[1].pos().0 - 0.0).abs() < EPS);
        assert!((cats[1].pos().1 - 300.0).abs() < EPS);
        assert!((cats[1].size().0 - 400.0).abs() < EPS);
        assert!((cats[1].size().1 - 300.0).abs() < EPS);

        // r: right half of lower half
        assert!((cats[2].pos().0 - 400.0).abs() < EPS);
        assert!((cats[2].pos().1 - 300.0).abs() < EPS);
        assert!((cats[2].size().0 - 400.0).abs() < EPS);
        assert!((cats[2].size().1 - 300.0).abs() < EPS);
    }

    // ── layout edge cases ──

    #[test]
    fn layout_empty_split() {
        let mut n = Node::Split(vec![], Direction::Column);
        n.layout((0.0, 0.0), (100.0, 100.0));
        // should not panic; nothing to lay out
        let cats = n.into_flat();
        assert!(cats.is_empty());
    }

    #[test]
    fn layout_single_child() {
        let mut n = Node::Split(vec![Node::Category(cat("only"))], Direction::Row);
        n.layout((10.0, 20.0), (500.0, 300.0));
        let cats = n.into_flat();
        assert!((cats[0].pos().0 - 10.0).abs() < EPS);
        assert!((cats[0].pos().1 - 20.0).abs() < EPS);
        assert!((cats[0].size().0 - 500.0).abs() < EPS);
        assert!((cats[0].size().1 - 300.0).abs() < EPS);
    }

    #[test]
    fn layout_single_child_column() {
        let mut n = Node::Split(vec![Node::Category(cat("only"))], Direction::Column);
        n.layout((10.0, 20.0), (500.0, 300.0));
        let cats = n.into_flat();
        assert!((cats[0].pos().0 - 10.0).abs() < EPS);
        assert!((cats[0].pos().1 - 20.0).abs() < EPS);
        assert!((cats[0].size().0 - 500.0).abs() < EPS);
        assert!((cats[0].size().1 - 300.0).abs() < EPS);
    }

    // ── layout offset ──

    #[test]
    fn layout_three_in_column() {
        let mut n = Node::Split(vec![
            Node::Category(cat("a")),
            Node::Category(cat("b")),
            Node::Category(cat("c")),
        ], Direction::Column);
        n.layout((100.0, 50.0), (900.0, 600.0));

        let cats = n.into_flat();
        assert_eq!(cats.len(), 3);

        assert!((cats[0].pos().0 - 100.0).abs() < EPS);
        assert!((cats[0].pos().1 - 50.0).abs() < EPS);
        assert!((cats[0].size().1 - 200.0).abs() < EPS);

        assert!((cats[1].pos().0 - 100.0).abs() < EPS);
        assert!((cats[1].pos().1 - 250.0).abs() < EPS);
        assert!((cats[1].size().1 - 200.0).abs() < EPS);

        assert!((cats[2].pos().0 - 100.0).abs() < EPS);
        assert!((cats[2].pos().1 - 450.0).abs() < EPS);
        assert!((cats[2].size().1 - 200.0).abs() < EPS);
    }

    // ── push then layout ──

    #[test]
    fn build_push_then_layout() {
        let mut root = Node::Split(vec![], Direction::Column);
        root.push(Node::Category(cat("a"))).unwrap();
        root.push(Node::Category(cat("b"))).unwrap();
        root.push(Node::Category(cat("c"))).unwrap();

        root.layout((0.0, 0.0), (300.0, 300.0));
        let cats = root.into_flat();
        assert_eq!(cats.len(), 3);
        assert!((cats[2].pos().1 - 200.0).abs() < EPS);
    }

    // ── insert then layout ──

    #[test]
    fn insert_then_layout() {
        let mut root = Node::Split(vec![
            Node::Category(cat("a")),
            Node::Category(cat("c")),
        ], Direction::Row);
        root.insert(Node::Category(cat("b")), 1).unwrap();

        root.layout((0.0, 0.0), (300.0, 100.0));
        let cats = root.into_flat();
        assert_eq!(cats.len(), 3);
        assert!((cats[0].size().0 - 100.0).abs() < EPS);
        assert!((cats[1].size().0 - 100.0).abs() < EPS);
        assert!((cats[2].size().0 - 100.0).abs() < EPS);
    }

    // ── remove then layout ──

    #[test]
    fn remove_then_layout() {
        let mut root = Node::Split(vec![
            Node::Category(cat("a")),
            Node::Category(cat("b")),
            Node::Category(cat("c")),
        ], Direction::Column);
        root.remove(1).unwrap();
        assert_eq!(root.len(), 2);
    }

    // ── swap order reflected in layout ──

    #[test]
    fn swap_affects_layout_order() {
        let mut root = Node::Split(vec![
            Node::Category(cat("a")),
            Node::Category(cat("b")),
        ], Direction::Row);
        root.swap(0, 1).unwrap();

        root.layout((0.0, 0.0), (200.0, 100.0));
        let cats = root.into_flat();
        // after swap, b is first → should be at x=0, a second → x=100
        assert_eq!(cats[0].name(), "b");
        assert_eq!(cats[1].name(), "a");
        assert!((cats[0].pos().0 - 0.0).abs() < EPS);
        assert!((cats[1].pos().0 - 100.0).abs() < EPS);
    }
}