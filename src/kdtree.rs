//! This KdTree implementation use `str::collections::BTreeMap` from the standard library

/// KdTree is essentially a binary tree with k-dimensions node
#[derive(Debug)]
struct KdTree<T, const K: usize>
where
    T: MultiDimension<K>,
{
    root: Link<T, K>,
}

#[derive(Debug)]
struct Node<T, const K: usize>
where
    T: MultiDimension<K>,
{
    value: T,
    left: Link<T, K>,
    right: Link<T, K>,
    depth: usize,
}

type Link<T, const K: usize> = Option<Box<Node<T, K>>>;

///
impl<T, const K: usize> Node<T, K>
where
    T: MultiDimension<K>,
{
    fn from_iter_help<I>(iter: I, depth: usize) -> Link<T, K>
    where
        I: IntoIterator<Item = T>,
    {
        let mut vec = iter.into_iter().collect::<Vec<_>>();
        if vec.len() == 0 {
            return None;
        }

        vec.sort_unstable_by(|a, b| a.nth_dim(depth % K).cmp(&b.nth_dim(depth % K)));

        // if length == 0, there shouldn't be any median:
        let mid = (vec.len() - 1) / 2;
        let mut it = vec.into_iter();

        let left = if mid != 0 {
            Node::from_iter_help(it.by_ref().take(mid), depth + 1)
        } else {
            None
        };

        let value = it.next()?;

        let right = Node::from_iter_help(it, depth + 1);

        Some(Box::new(Self {
            value,
            left,
            right,
            depth,
        }))
    }
}

///
impl<T, const K: usize> FromIterator<T> for KdTree<T, K>
where
    T: MultiDimension<K>,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        KdTree {
            root: Node::from_iter_help(iter, 0),
        }
    }
}

pub trait MultiDimension<const K: usize> {
    type Dimension: Ord;
    fn nth_dim(&self, n: usize) -> Self::Dimension;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        impl MultiDimension<1> for i32 {
            type Dimension = Self;
            fn nth_dim(&self, _: usize) -> Self::Dimension {
                return *self;
            }
        }
        let nodes = [0, 1, 2, 3, 4, 5];
        let tree = nodes.into_iter().collect::<KdTree<i32, 1>>();
        println!("{:?}", tree);
    }

    #[test]
    fn two_dim() {
        impl MultiDimension<2> for (isize, isize) {
            type Dimension = isize;
            fn nth_dim(&self, n: usize) -> Self::Dimension {
                if n % 2 == 0 {
                    self.0
                } else {
                    self.1
                }
            }
        }

        let nodes =
            [(40, 45), (70, 10), (15, 70), (69, 50), (66, 85), (85, 90)] as [(isize, isize); 6];
        let tree = nodes.into_iter().collect::<KdTree<_, 2>>();
        println!("{:?}", tree);
    }
}
