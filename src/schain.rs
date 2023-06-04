use std::error::Error;
#[macro_use]
use crate::Segment;
use crate::utils::{Seg, SegT};

#[derive(PartialEq, PartialOrd)]
#[derive(Debug)]
pub struct SChain {
    chain: Vec<Segment>
}

impl Default for SChain {
    fn default() -> Self {
        Self {
            chain: Vec::new(),
        }
    }
}

impl PartialEq<Segment> for SChain {
    fn eq(&self, other: &Segment) -> bool {
        self.chain.len()==1 && self.chain.first().unwrap() == other
    }
}

impl PartialOrd<Segment> for SChain {
    fn partial_cmp(&self, other: &Segment) -> Option<std::cmp::Ordering> {
        return match self.chain.first() {
            Some(x) => x.partial_cmp(&other),
            None => Some(std::cmp::Ordering::Less),
        };
    }
}

impl SChain {
    pub fn new() -> Self{
        Self {
            chain: Vec::new(),
        }
    }

    pub fn push(&mut self, seg: Segment) -> Result<(),Box<dyn Error>> {
        if !self.chain.is_empty() && seg < *self.chain.last().unwrap(){
            return Err("seg < chain.last".into());
        }
        self.chain.push(seg);
        Ok(())
    }
    pub fn is_empty(&self) -> bool { self.chain.is_empty() }
}

impl Seg for SChain {
    fn contains(&self, value: u32) -> bool {
        self.chain.iter().any(|interval| interval.contains(value))
    }
    fn start(&self) -> Option<&u32> {
        self.chain.first().unwrap().start()
    }
    fn end(&self) -> Option<&u32> {
        self.chain.last().unwrap().end()
    }
    fn empty(&self) -> bool {
        self.chain.is_empty()
    }

    fn set_start(&mut self, new_start: u32) -> Result<(),Box<dyn Error>>{
        if new_start > *self.end().unwrap() {
            return Err("new end < start".into());
        }
        if new_start <= *self.start().unwrap() {
            self.chain[0].set_start(new_start);
        } else {
            let new_data = self
                .chain
                .iter()
                .skip_while(|&seg| *seg.start().unwrap() < new_start)
                .cloned()
                .collect();
            self.chain = new_data;
        }
        Ok(())
    }
    fn set_end(&mut self, end: u32) -> Result<(),Box<dyn Error>>{
        // todo
        Ok(())
    }
}

impl SegT<SChain> for SChain {
    type Output = SChain;

    fn intersect(&self, other: &SChain) -> Option<Self::Output> {
        let mut result = SChain::new();

        let mut i = 0;
        let mut j = 0;

        while i < self.chain.len() && j < other.chain.len() {
            let seg1 = &self.chain[i];
            let seg2 = &other.chain[j];

            // get intersection
            match seg1.intersect(seg2){
                Some(x) => result.push(x).unwrap(),
                None => (),
            }

            // Move to the next interval in the chain that has a smaller endpoint
            if seg1.end() < seg2.end() {
                i += 1;
            } else {
                j += 1;
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    fn union(&self, other: &SChain) -> Option<Self::Output> {
        None
    }

    fn overlap(&self, other: &SChain) -> bool {
        self.chain.iter().any(|interval| other.overlap(interval))
    }

    fn strict_overlap(&self, other: &SChain) -> bool {
        self.chain.iter().any(|interval| other.strict_overlap(interval))
    }
}

impl SegT<Segment> for SChain {
    type Output = SChain;

    fn intersect(&self, other: &Segment) -> Option<Self::Output> {
        let mut result = SChain::new();

        let mut i = 0;

        while i < self.chain.len() {
            let seg = &self.chain[i];

            // get intersection
            match seg.intersect(other){
                Some(x) => result.push(x).unwrap(),
                None => (),
            }

            // Move to the next interval in the chain that has a smaller endpoint
            if seg.end() < other.end() {
                i += 1;
            } else {
                break;
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    fn union(&self, other: &Segment) -> Option<Self::Output> {
        None
    }

    fn overlap(&self, other: &Segment) -> bool {
        let os = if let Some(os) = other.start() { *os } else { return false; };
        let oe = if let Some(oe) = other.end() { *oe } else { return false; };
        let ss = if let Some(ss) = self.start() { *ss } else { return false; };
        let se = if let Some(se) = self.end() { *se } else { return false; };
        ss <= oe && os <= se
    }

    fn strict_overlap(&self, other: &Segment) -> bool {
        self.chain.iter().any(|interval| interval.overlap(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_create_chain() -> Result<(),Box<dyn Error>> {
        let mut chain = SChain::new();
        assert!(chain.is_empty());
        chain.push(seg!(1,5)?)?;
        chain.push(seg!(10,15)?)?;
        chain.push(seg!(20,25)?)?;
        let chain_full = SChain{chain:vec![seg!(1,5)?,seg!(10,15)?,seg!(20,25)?]};
        assert_eq!(chain,chain_full);

        Ok(())
    }

    #[test]
    fn test_intersect_chain_non_empty() -> Result<(),Box<dyn Error>> {
        let chain1 = SChain{chain:vec![seg!(1,5)?,seg!(10,15)?,seg!(20,25)?]};
        let chain2 = SChain{chain:vec![seg!(5, 12)?,seg!(18, 22)?,seg!(30, 35)?]};
        let chain3 = SChain{chain:vec![seg!(5,5)?,seg!(10,12)?,seg!(20,22)?]};

        let result = chain1.intersect(&chain2);

        assert!(result.is_some());

        let intersected_chain = result.unwrap();
        assert_eq!(intersected_chain, chain3);
        Ok(())
    }

    #[test]
    fn test_intersect_chain_empty() -> Result<(),Box<dyn Error>> {
        let chain1 = SChain {
            chain: vec![seg!(1, 5)?, seg!(10, 15)?, seg!(20, 25)?],
        };
        let chain2 = SChain {
            chain: vec![seg!(30, 35)?, seg!(40, 45)?],
        };

        let result = chain1.intersect(&chain2);

        assert!(result.is_none());

        Ok(())
    }

    #[test]
    fn test_intersect_seg_non_empty() -> Result<(),Box<dyn Error>> {
        let chain = SChain{chain:vec![seg!(1,5)?,seg!(10,15)?,seg!(20,25)?]};
        let sgt = seg!(3,14)?;
        let res = SChain{chain:vec![seg!(3,5)?,seg!(10,14)?]};

        let result = chain.intersect(&sgt);

        assert!(result.is_some());

        let intersected_chain = result.unwrap();
        assert_eq!(intersected_chain, res);
        Ok(())
    }

    #[test]
    fn test_eq() -> Result<(),Box<dyn Error>> {
        let chain1 = SChain{chain:vec![seg!(1,5)?,seg!(10,15)?,seg!(20,25)?]};
        let chain2 = SChain{chain:vec![seg!(1,5)?,seg!(10,15)?,seg!(20,25)?]};
        assert_eq!(chain1,chain2);

        let chain1 = SChain::new();
        let chain2 = SChain::new();
        assert_eq!(chain1,chain2);

        Ok(())
    }

    #[test]
    fn test_ne() -> Result<(),Box<dyn Error>> {
        let chain1 = SChain{chain:vec![seg!(1,5)?,seg!(10,15)?,seg!(20,25)?]};
        let chain2 = SChain{chain:vec![seg!(10,15)?,seg!(20,25)?]};
        assert_ne!(chain1, chain2);

        let chain1 = SChain::new();
        let chain2 = SChain{chain:vec![seg!(10,15)?,seg!(20,25)?]};
        assert_ne!(chain1, chain2);

        Ok(())
    }

    #[test]
    fn test_lt() -> Result<(),Box<dyn Error>> {
        let chain1 = SChain{chain:vec![seg!(1,5)?,seg!(10,15)?,seg!(20,25)?]};
        let chain2 = SChain{chain:vec![seg!(10,15)?,seg!(20,25)?]};
        assert!(chain1 < chain2);

        let chain1 = SChain{chain:vec![seg!(1,5)?,seg!(10,15)?,seg!(20,25)?]};
        let chain2 = SChain{chain:vec![seg!(2,5)?,seg!(10,15)?,seg!(20,25)?]};
        assert!(chain1 < chain2);

        let chain1 = SChain{chain:vec![seg!(1,5)?,seg!(10,15)?,seg!(20,25)?]};
        let chain2 = SChain{chain:vec![seg!(11,12)?]};
        assert!(chain1 < chain2);

        let chain1 = SChain{chain:vec![seg!(1,5)?,seg!(10,15)?,seg!(20,25)?]};
        let seg1 = seg!(11,12)?;
        assert!(chain1 < seg1);

        Ok(())
    }

    #[test]
    fn test_overlap_seg() -> Result<(),Box<dyn Error>> {
        let s1 = seg!(1,10)?;
        let c1 = SChain{chain:vec![seg!(1,5)?,seg!(10,15)?,seg!(20,25)?]};
        assert!(c1.overlap(&s1));

        let s1 = seg!(7,9)?;
        assert!(c1.overlap(&s1));

        let s1 = seg!(25,29)?;
        assert!(c1.overlap(&s1));

        let s1 = seg!(26,29)?;
        assert!(!c1.overlap(&s1));
        Ok(())
    }

    #[test]
    fn test_overlap_schain() -> Result<(),Box<dyn Error>> {
        let c1 = SChain{chain:vec![seg!(1,5)?,seg!(10,15)?,seg!(20,25)?]};
        let c2 = SChain{chain:vec![seg!(4,7)?,seg!(17,23)?]};
        assert!(c1.overlap(&c2));

        let c2 = SChain{chain:vec![seg!(7,9)?,seg!(16,18)?,seg!(26,29)?]};
        assert!(c1.overlap(&c2));

        let c2 = SChain{chain:vec![seg!(26,30)?]};
        assert!(!c1.overlap(&c2));
        Ok(())
    }

    #[test]
    fn test_strict_overlap_seg() -> Result<(),Box<dyn Error>> {
        let s1 = seg!(1,10)?;
        let c1 = SChain{chain:vec![seg!(2,5)?,seg!(10,15)?,seg!(20,25)?]};
        assert!(c1.strict_overlap(&s1));

        let s1 = seg!(7,9)?;
        assert!(!c1.strict_overlap(&s1));

        let s1 = seg!(25,29)?;
        assert!(c1.strict_overlap(&s1));

        let s1 = seg!(1,30)?;
        assert!(c1.strict_overlap(&s1));

        let s1 = seg!(26,29)?;
        assert!(!c1.strict_overlap(&s1));
        Ok(())
    }

    #[test]
    fn test_strict_overlap_schain() -> Result<(),Box<dyn Error>> {
        let c1 = SChain{chain:vec![seg!(1,5)?,seg!(10,15)?,seg!(20,25)?]};
        let c2 = SChain{chain:vec![seg!(4,7)?,seg!(17,23)?]};
        assert!(c1.strict_overlap(&c2));

        let c2 = SChain{chain:vec![seg!(7,9)?,seg!(16,18)?,seg!(26,29)?]};
        assert!(!c1.strict_overlap(&c2));

        let c2 = SChain{chain:vec![seg!(26,30)?]};
        assert!(!c1.strict_overlap(&c2));
        Ok(())
    }
}
