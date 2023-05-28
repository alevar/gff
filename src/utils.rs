pub trait SegT<T>{
    type Output;

    fn contains(&self,pos:u32) -> bool; // the segment contains a given position
    fn intersect(&self, other: &T) -> Option<Self::Output>; // intersect current segment with another. Return a segment of the current type with the intersection of the two. Should be able to intersect a chain with an interval and vice versa and same types
}