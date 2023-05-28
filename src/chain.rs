use std::error::Error;
use crate::Segment;
use crate::utils::SegT;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Chain {
    chain: Vec<Segment>
}

impl Default for Chain {
    fn default() -> Self {
        Chain {
            chain: Vec::new(),
        }
    }
}

impl Chain {
    pub fn new() -> Result<Self, Box<dyn Error>>{
        Ok(Self {
            chain: Vec::new(),
        })
    }

    pub fn push(&mut self, seg: Segment){
        self.chain.push(seg);
    }
}

impl SegT<Chain> for Chain {
    type Output = Chain;

    fn contains(&self, value: u32) -> bool {
        true
    }

    fn intersect(&self, other: &Chain) -> Option<Self::Output> {
        None
    }
}

macro_rules! Chain {
    ($($x:expr),*) => {{
        let mut temp = Chain::new();
        $(
            temp.push($x);
        )*
        temp
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_chain() {
        let chain = Chain::new().unwrap();
    }
}
