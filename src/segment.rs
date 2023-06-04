use std::error::Error;
use crate::schain::SChain;
#[macro_use]
use crate::schain;
use crate::utils::{Seg, SegT};

#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
#[derive(Debug)]
pub struct Segment {
    start: u32,
    end: u32,
}

impl Default for Segment {
    fn default() -> Self {
        Segment {
            start: u32::MAX,
            end: u32::MAX,
        }
    }
}

impl From<Segment> for (u32, u32) {
    fn from(seg: Segment) -> Self {
        (seg.start, seg.end)
    }
}

impl Segment {
    pub fn new(start:u32, end:u32) -> Result<Self, Box<dyn Error>>{
        if start > end{
            return Err("start > end".into());
        }
        Ok(Self {
            start,
            end,
        })
    }

    pub fn set_start(&mut self,start:u32) -> Result<(),Box<dyn Error>>{
        if start > self.end{
            return Err("start > end".into());
        }
        self.start = start;
        Ok(())
    }

    pub fn set_end(&mut self,end:u32) -> Result<(),Box<dyn Error>>{
        if self.start > end{
            return Err("start > end".into());
        }
        self.end = end;
        Ok(())
    }

    pub fn slen(&self) -> u32{
        return (self.end + 1) - self.start;
    }
}

impl Seg for Segment {
    fn contains(&self, value: u32) -> bool {
        value >= self.start && value <= self.end
    }
    fn start(&self) -> Option<&u32> {
        if self.empty() { None } else { Some(&self.start) }
    }
    fn end(&self) -> Option<&u32> {
        if self.empty() { None } else { Some(&self.end) }
    }
    fn empty(&self) -> bool {
        self.end == self.start && self.start == u32::MAX
    }

    fn set_start(&mut self, start: u32) -> Result<(),Box<dyn Error>>{
        if start > self.end {
            return Err("new start > end".into());
        }
        self.start = start;
        Ok(())
    }
    fn set_end(&mut self, end: u32) -> Result<(),Box<dyn Error>>{
        if end < self.start {
            return Err("new end < start".into());
        }
        self.end = end;
        Ok(())
    }
}

impl SegT<Segment> for Segment {
    type Output = Segment;

    fn intersect(&self, other: &Segment) -> Option<Self::Output> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        if start <= end {
            Some(Segment::new(start, end).unwrap())
        } else {
            None
        }
    }

    fn union(&self, other: &Segment) -> Option<Self::Output> {
        match self.overlap(other){
            true => {
                let start = std::cmp::min(self.start, other.start);
                let end = std::cmp::max(self.end, other.end);
                Some(Segment { start, end })
            },
            false => None,
        }
    }

    fn overlap(&self, other: &Segment) -> bool {
        let os = if let Some(os) = other.start() { *os } else { return false; };
        let oe = if let Some(oe) = other.end() { *oe } else { return false; };
        self.start <= oe && os <= self.end
    }

    fn strict_overlap(&self, other: &Segment) -> bool {
        self.overlap(other)
    }
}

impl SegT<SChain> for Segment {
    type Output = SChain;

    fn intersect(&self, other: &SChain) -> Option<Self::Output> {
        Some(SChain::new())
    }

    fn union(&self, other: &SChain) -> Option<Self::Output> {
        None
    }

    fn overlap(&self, other: &SChain) -> bool{
        let os = if let Some(os) = other.start() { *os } else { return false; };
        let oe = if let Some(oe) = other.end() { *oe } else { return false; };
        self.start <= oe && os <= self.end
    }

    fn strict_overlap(&self, other: &SChain) -> bool {
        other.strict_overlap(self)
    }
}

#[macro_export]
macro_rules! seg {
    ($start:expr,$end:expr) => {{
        Segment::new($start,$end)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_segtp() {
        let segtp = Segment::new(1, 5).unwrap();
        assert_eq!(segtp.start, 1);
        assert_eq!(segtp.end, 5);

        let segtp_macro = seg!(1, 5).unwrap();
        assert_eq!(segtp,segtp_macro);
    }

    #[test]
    fn test_create_invalid_segtp() {
        assert!(Segment::new(5, 1).is_err());
    }

    #[test]
    fn test_intersect() {
        let segtp1 = Segment::new(1, 5).unwrap();
        let segtp2 = Segment::new(3, 7).unwrap();
        let segtp3 = Segment::new(6, 10).unwrap();
        let segtp4 = Segment::new(0, 3).unwrap();
        let segtp5 = Segment::new(7, 9).unwrap();

        assert_eq!(Segment::intersect(&segtp1, &segtp2), Some(Segment::new(3, 5).unwrap()));
        assert_eq!(segtp1.intersect(&segtp3), None);
        assert_eq!(segtp1.intersect(&segtp4), Some(Segment::new(1, 3).unwrap()));
        assert_eq!(segtp1.intersect(&segtp5), None);
    }

    #[test]
    fn test_eq(){
        let s1 = seg!(1,10);
        let s1 = match s1 {
            Ok(x) => x,
            Err(_) => panic!("fail"),
        };
        let s2 = seg!(1,10);
        let s2 = match s2 {
            Ok(x) => x,
            Err(_) => panic!("fail"),
        };
        assert_eq!(s1,s2);
    }

    #[test]
    fn test_lt(){
        let s1 = seg!(1,10);
        let s1 = match s1 {
            Ok(x) => x,
            Err(_) => panic!("fail"),
        };
        let s2 = seg!(2,10);
        let s2 = match s2 {
            Ok(x) => x,
            Err(_) => panic!("fail"),
        };
        assert!(s1<s2);

        let s1 = seg!(1,10);
        let s1 = match s1 {
            Ok(x) => x,
            Err(_) => panic!("fail"),
        };
        let s2 = seg!(2,9);
        let s2 = match s2 {
            Ok(x) => x,
            Err(_) => panic!("fail"),
        };
        assert!(s1<s2);

        let s1 = seg!(1,10);
        let s1 = match s1 {
            Ok(x) => x,
            Err(_) => panic!("fail"),
        };
        let s2 = seg!(1,9);
        let s2 = match s2 {
            Ok(x) => x,
            Err(_) => panic!("fail"),
        };
        assert!(s1>s2);
    }

    #[test]
    fn test_overlap_seg() -> Result<(),Box<dyn Error>> {
        let s1 = seg!(1,10)?;
        let s2 = seg!(5,15)?;
        assert!(s1.overlap(&s2));

        let s2 = seg!(10,15)?;
        assert!(s1.overlap(&s2));

        let s2 = seg!(11,15)?;
        assert!(!s1.overlap(&s2));
        Ok(())
    }

    #[test]
    fn test_overlap_schain() -> Result<(),Box<dyn Error>> {
        let s1 = seg!(1,10)?;
        let mut c2 = SChain::new();
        c2.push(seg!(1,4)?)?;
        c2.push(seg!(5,11)?)?;
        c2.push(seg!(15,24)?)?;

        assert!(s1.overlap(&c2));

        let mut c2 = SChain::new();
        c2.push(seg!(11,13)?)?;
        c2.push(seg!(15,24)?)?;

        assert!(!s1.overlap(&c2));

        let mut c2 = SChain::new();
        c2.push(seg!(10,13)?)?;
        c2.push(seg!(15,24)?)?;

        assert!(s1.overlap(&c2));

        let mut c2 = SChain::new();
        c2.push(seg!(10,13)?)?;
        c2.push(seg!(18,24)?)?;

        let s1 = seg!(14,17)?;
        assert!(s1.overlap(&c2));
        Ok(())
    }

    fn test_strict_overlap_seg() -> Result<(),Box<dyn Error>> {
        let s1 = seg!(1,10)?;
        let s2 = seg!(5,15)?;
        assert!(s1.strict_overlap(&s2));

        let s2 = seg!(10,15)?;
        assert!(s1.strict_overlap(&s2));

        let s2 = seg!(11,15)?;
        assert!(!s1.strict_overlap(&s2));
        Ok(())
    }

    #[test]
    fn test_strict_overlap_schain() -> Result<(),Box<dyn Error>> {
        let s1 = seg!(1,10)?;
        let mut c2 = SChain::new();
        c2.push(seg!(1,4)?)?;
        c2.push(seg!(5,11)?)?;
        c2.push(seg!(15,24)?)?;

        assert!(s1.strict_overlap(&c2));

        let mut c2 = SChain::new();
        c2.push(seg!(11,13)?)?;
        c2.push(seg!(15,24)?)?;

        assert!(!s1.strict_overlap(&c2));

        let mut c2 = SChain::new();
        c2.push(seg!(10,13)?)?;
        c2.push(seg!(15,24)?)?;

        assert!(s1.strict_overlap(&c2));

        let mut c2 = SChain::new();
        c2.push(seg!(10,13)?)?;
        c2.push(seg!(18,24)?)?;

        let s1 = seg!(14,17)?;
        assert!(!s1.strict_overlap(&c2));
        Ok(())
    }
}
