use std::error::Error;

pub trait Seg{ // type invariant traits
    fn contains(&self,pos:u32) -> bool; // the segment contains a given position
    fn start(&self) -> Option<&u32>; // returns the start of the segment
    fn end(&self) -> Option<&u32>; // returns the end of the segment

    fn set_start(&mut self, start: u32) -> Result<(),Box<dyn Error>>; // set new start
    fn set_end(&mut self, end: u32) -> Result<(),Box<dyn Error>>; // set new end
    fn empty(&self) -> bool;
}

pub trait SegT<T>: Seg{ // type specific traits
    type Output;

    fn intersect(&self, other: &T) -> Option<Self::Output>; // intersect current segment with another. Return a segment of the current type with the intersection of the two. Should be able to intersect a chain with an interval and vice versa and same types
    fn union(&self, other: &T) -> Option<Self::Output>; // perform union on the currect segment with another. Return result of the union
    fn overlap(&self, other: &T) -> bool; // returns true if two intervals overlap one another
    fn strict_overlap(&self, other: &T) -> bool; // implementation depends on use case, however intended purpose is for collections of intervals, returns true only if any individual intervals within the collections overlap, not just start end coordinates of the entire collection. For instance (0,10),(20,30) overlap (14,16) should not return strict overlap true
}