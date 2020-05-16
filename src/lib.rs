//! # SlotVec
//!
//! SlotVec is a Vec where you can take out and replace values without increasing the
//! size of the map.


use std::ops::{Index, IndexMut};
use std::iter::{Iterator, IntoIterator};

#[derive(Debug)]
struct Collection {
    inner: Vec<Option<u8>>,
    state: CollectionState,
}

impl Collection {
    fn new() -> Self {
        Self {
            inner: Vec::new(),
            state: CollectionState::Empty,
        }
    }

    fn add(&mut self, item: u8) -> usize {
        match self.state {
            CollectionState::Empty => {
                self.state = CollectionState::Full(1);
                self.inner.push(Some(item));
                0
            }

            CollectionState::Full(n) => {
                self.state = CollectionState::Full(n + 1);
                self.inner.push(Some(item));
                n as usize
            }

            CollectionState::NotFull(n, avail) => {
                let avail = avail - 1;

                for i in 0..n {
                    let slot = &mut self.inner[i as usize];
                    if slot.is_none() {
                        *slot = Some(item);

                        if avail > 0 {
                            self.state = CollectionState::NotFull(n+1, avail);
                        } else {
                            self.state = CollectionState::Full(n + 1);
                        }

                        return i as usize;
                    }
                }

                panic!("Collection notfull, but no available slot found!");
                
            }
        }
    }

    fn take(&mut self, index: usize) -> u8 {
        let item = self[index].take().unwrap();

        match self.state {
            CollectionState::Full(n) => self.state = CollectionState::NotFull(n - 1, 1),
            CollectionState::NotFull(n, avail) => self.state = CollectionState::NotFull(n - 1, avail + 1),
            _ => (),
        }

        item

    }

    fn len(&self) -> u32 {
        match self.state {
            CollectionState::Full(n) => n,
            CollectionState::NotFull(n, _) => n,
            _ => 0,
        }
    }

    fn is_empty(&self) -> bool {
        match self.state {
            CollectionState::Empty => true,
            CollectionState::NotFull(n, _) => n == 0,
            _ => false,
        }
    }
}


impl Index<usize> for Collection {
    type Output = Option<u8>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for Collection {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl IntoIterator for Collection {
    type Item = u8;
    type IntoIter = CollectionIter;

    fn into_iter(self) -> Self::IntoIter {
        CollectionIter {
            inner: self,
            pos: 0,
        }
    }
}

struct CollectionIter {
    inner: Collection,
    pos: usize,
}

impl Iterator for CollectionIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let limit = self.inner.inner.len() - 1;
        if self.pos > limit {
            return None;
        };

        while self.inner.inner[self.pos].is_none() {
            self.pos += 1;
            if self.pos > limit {
                return None;
            }
        }

        // We know it's `Some`
        let res= self.inner.inner[self.pos];
        self.pos += 1;
        res
    }
}

#[derive(Debug)]
enum CollectionState {
    Empty,
    Full(u32),
    NotFull(u32, u32),

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations_doesnt_panic() {
        let mut mycoll = Collection::new();

        mycoll.add(1);
        mycoll.add(2);
        mycoll.add(3);
    
        println!("{:?}", mycoll);
    
        let test = mycoll[1];
    
        println!("test = {:?}", test);
    
        let test2 = mycoll.take(1);
        println!("{:?}", mycoll);
        println!("test = {:?}", test2);
    
        let index = mycoll.add(4);
        mycoll.add(5);
        println!("test = {:?}", mycoll);
        
        mycoll.take(index);
        println!("test = {:?}", mycoll);
        for item in mycoll {
            println!("{}", item);
        }
    }
}