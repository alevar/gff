use std::error::Error;
use crate::chain::Chain;
use crate::utils::SegT;

#[derive(PartialEq)]
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

impl SegT<Segment> for Segment {
    type Output = Segment;

    fn contains(&self, value: u32) -> bool {
        value >= self.start && value <= self.end
    }

    fn intersect(&self, other: &Segment) -> Option<Self::Output> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);

        if start <= end {
            Some(Segment::new(start, end).unwrap())
        } else {
            None
        }
    }
}

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
}
