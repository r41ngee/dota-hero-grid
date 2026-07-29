use crate::*;

pub enum Direction {
    Row,
    Column,
}

pub enum Node {
    Category(Category),
    Split(Vec<Node>, Direction)
}

pub const GRID_SIZE: (f32, f32) = (1000.0, 500.0);

impl Node {
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

    pub fn push(&mut self, node: Node) -> Result<(), &'static str> {
        match self {
            Node::Category(_) => Err("Tried to push a node into a category"),
            Node::Split(v, _) => { v.push(node); Ok(()) }
        }
    }
}