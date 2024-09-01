use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Object {
    pub name: String,
}

pub struct Relations {
    pub relations: Vec<(Object, Object)>,
}

pub struct Graph {
    pub relations: Relations,
    pub positions: HashMap<Object, (usize, usize)>,
}
