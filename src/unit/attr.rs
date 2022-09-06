use std::collections::HashMap;

type Key = &'static str;
type Value = i32;

#[derive(Clone, Default)]
pub struct Attr {
    map: HashMap<Key, Value>,
}

impl Attr {
    pub fn new(hp: Value, att: Value) -> Self {
        Self {
            map: HashMap::from([("hp", hp), ("max_hp", hp), ("att", att)]),
        }
    }

    pub fn apply(&mut self, delta: &Self) {
        for (k, v) in delta.map.iter() {
            let v = *self.map.get(k).unwrap_or(&0) + v;
            self.map.insert(k, v);
        }
    }

    pub fn get(&self, key: Key) -> Value {
        *self.map.get(key).unwrap_or(&0)
    }
}

impl FromIterator<(Key, Value)> for Attr {
    fn from_iter<T: IntoIterator<Item = (Key, Value)>>(iter: T) -> Self {
        Self {
            map: HashMap::from_iter(iter),
        }
    }
}

impl<const N: usize> From<[(Key, Value); N]> for Attr {
    fn from(arr: [(Key, Value); N]) -> Self {
        Self {
            map: HashMap::from(arr),
        }
    }
}
