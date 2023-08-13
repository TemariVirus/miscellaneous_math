use crate::Scalar;

#[derive(Clone, Debug)]
pub struct Point<T>(Vec<T>)
where
    T: Scalar;

impl<T> Point<T>
where
    T: Scalar,
{
    pub fn new(data: &[T]) -> Self {
        Point(data.to_vec())
    }

    pub fn get(&self, index: usize) -> T {
        self.0[index].clone()
    }

    pub fn set(&mut self, index: usize, value: T) {
        self.0[index] = value;
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn distance_squared(&self, other: &Self) -> T {
        assert_eq!(self.len(), other.len());

        let mut sum = T::zero();
        for i in 0..self.len() {
            let diff = self.get(i) - other.get(i);
            sum += diff * diff;
        }
        sum
    }
}
