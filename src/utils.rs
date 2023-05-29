use std::error::Error;

pub trait Seg{ // type invariant traits
    fn contains(&self,pos:u32) -> bool; // the segment contains a given position
    fn start(&self) -> Option<&u32>; // returns the start of the segment
    fn end(&self) -> Option<&u32>; // returns the end of the segment

    fn set_start(&mut self, start: u32) -> Result<(),Box<dyn Error>>; // set new start
    fn set_end(&mut self, end: u32) -> Result<(),Box<dyn Error>>; // set new end
}

pub trait SegT<T>: Seg{ // type specific traits
    type Output;

    fn intersect(&self, other: &T) -> Option<Self::Output>; // intersect current segment with another. Return a segment of the current type with the intersection of the two. Should be able to intersect a chain with an interval and vice versa and same types
    fn union(&self, other: &T) -> Option<Self::Output>; // perform union on the currect segment with another. Return result of the union
}