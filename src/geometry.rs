use crate::*;

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

pub const GRID_SIZE: (f32, f32) = (1000.0, 500.0);

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
                let heigth = size.1;
                match d {
                    Direction::Column => {
                        let per = heigth / len as f32;
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
                                (per, heigth)
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
            Node::Split(v, _) => if v.is_empty() { Some(v) } else { None },
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
                let max_idx = v.len() - 1;
                if max_idx < idx {
                    Err("Index is bigger than vector")
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
                let max_idx = v.len() - 1;
                if max_idx < idx {
                    Err("Index is bigger than vector")
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
    pub fn len(&self) -> Result<usize, &str> {
        match self {
            Node::Category(_) => Err("Tried to check length of a category"),
            Node::Split(v, _) => {
                Ok(v.len())
            }
        }
    }

    /// Method used to determine
    /// if container is empty.
    pub fn is_empty(&self) -> Result<bool, &str> {
        match self {
            Node::Category(_) => Err("Category cannot be empty"),
            Node::Split(v, _) => Ok(v.is_empty())
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