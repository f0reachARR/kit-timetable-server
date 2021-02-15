use std::marker::PhantomData;

#[derive(Copy, Clone, Debug, Hash, Default, PartialEq, Eq, Ord, PartialOrd)]
pub struct Phantom<T, V = u32>
where
    V: Copy + Clone,
{
    data: V,
    _phantom: PhantomData<T>,
}

impl<T, V: Copy + Clone> Phantom<T, V> {
    pub fn new(data: V) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }

    pub fn get(&self) -> V {
        self.data
    }
}

impl<T, V: Copy + Clone> From<V> for Phantom<T, V> {
    fn from(data: V) -> Self {
        Self {
            data,
            _phantom: PhantomData,
        }
    }
}