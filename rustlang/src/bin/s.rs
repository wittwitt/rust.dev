use std::collections::hash_map::{HashMap, Iter};

fn main() {
    gc1(&HashMap::new());
    gc2(&HashMap::new());

    let mut c = HashMap::new();
    c.insert("string", 1);

    // gc3(c);
}

// Works
fn gc1<'a>(map: &'a dyn GroupedCollection<'a, usize, usize, Iter<'a, usize, Vec<usize>>>) {
    let _ = map.iter().collect::<Vec<_>>();
}

// Works
fn gc2<'a, M>(map: &'a M)
where
    M: 'a + GroupedCollection<'a, usize, usize, Iter<'a, usize, Vec<usize>>>,
{
    let _ = map.iter().collect::<Vec<_>>();
}

fn gc3<M>(map: M)
where
    M: for<'a> GroupedCollection<'a, usize, usize, Iter<'a, usize, Vec<usize>>>,
{
    let _ = map.iter().collect::<Vec<_>>();
}

pub trait GroupedCollection<'a, K, V, I: 'a> {
    fn iter(&'a self) -> I;
}

impl<'a, K, V> GroupedCollection<'a, K, V, Iter<'a, K, Vec<V>>> for HashMap<K, Vec<V>> {
    fn iter(&'a self) -> Iter<'a, K, Vec<V>> {
        HashMap::iter(&self)
    }
}
