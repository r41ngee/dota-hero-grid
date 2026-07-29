use crate::*;

pub(crate) const GRID_SIZE: (f32, f32) = (1200.0, 800.0);

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub enum Value {
    Category(Category),
    Row(Vec<Object>),
    Column(Vec<Object>),
}

#[derive(Clone)]
pub struct Object {
    pub corners: (Point, Point),
    value: Value,
}

impl Object {
    pub fn root(value: Value) -> Self {
        Self { corners: (
                Point { x: 0.0, y: 0.0},
                Point { x: GRID_SIZE.0, y: GRID_SIZE.1 }
            ),
            value,
        }
    }

    pub fn new(
        corners: (Point, Point),
        value: Value,
    ) -> Self {
        Self { corners, value }
    }

    pub fn push(&mut self, new: Value) {
        let self_width = self.width();
        let self_height = self.height();
        match self.value {
            Value::Column(ref mut v) => {
                let len = v.len() + 1;
                let height = self_height;
                let height_per = height / len as f32;

                let object = Object::new(
                    (
                        Point { x: self.corners.0.x, y: height - height_per},
                        Point { x: self.corners.1.x, y: height},
                    ),
                    new
                );

                v.push(object);
                for (idx, item) in v.iter_mut().enumerate() {
                    let lower = height_per * idx as f32;
                    let higher = height_per * (idx as f32 + 1f32);
                    item.corners.0.y = lower;
                    item.corners.1.y = higher;
                }
            },
            Value::Row(ref mut v) => {
                let len = v.len() + 1;
                let width = self_width;
                let width_per = width / len as f32;

                let object = Object::new(
                    (
                        Point { y: self.corners.0.y, x: width - width_per},
                        Point { y: self.corners.1.y, x: width},
                    ),
                    new
                );

                v.push(object);
                for (idx, item) in v.iter_mut().enumerate() {
                    let lower = width_per * idx as f32;
                    let higher = width_per * (idx as f32 + 1f32);
                    item.corners.0.x = lower;
                    item.corners.1.x = higher;
                }
            },
            Value::Category(_) => panic!()
        }
    }

    fn height(&self) -> f32 {
        (self.corners.0.y - self.corners.1.y).abs()
    }

    fn width(&self) -> f32 {
        (self.corners.0.x - self.corners.1.x).abs()
    }

    pub fn into_categories(&self) -> Vec<Category> {
        match &self.value {
            Value::Category(c) => {
                vec![c.clone()]
            },
            Value::Column(v) | Value::Row(v) => {
                v.iter().map(|x| {
                    x.into_categories()
                }).flatten().collect()
            }
        }
    }
}