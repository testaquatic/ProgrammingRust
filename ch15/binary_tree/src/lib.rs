use BinaryTree::*;

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

struct TreeIter<'a, T> {
    unvisited: Vec<&'a TreeNode<T>>,
}

impl<'a, T: 'a> TreeIter<'a, T> {
    fn push_left_edge(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<T> BinaryTree<T> {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.push_left_edge(self);
        iter
    }
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match self {
            BinaryTree::Empty => {
                *self = NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut node) => {
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

impl<'a, T: 'a> IntoIterator for &'a BinaryTree<T> {
    type IntoIter = TreeIter<'a, T>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.unvisited.pop()?;
        self.push_left_edge(&node.right);
        Some(&node.element)
    }
}

#[cfg(test)]
mod tests {
    use crate::BinaryTree;

    #[test]
    fn test_into_iter() {
        let mut tree = BinaryTree::Empty;
        tree.add("jaeger");
        tree.add("robot");
        tree.add("droid");
        tree.add("mecha");

        let mut v = Vec::new();
        for kind in &tree {
            v.push(*kind);
        }
        assert_eq!(v, ["droid", "jaeger", "mecha", "robot"]);

        assert_eq!(
            tree.iter()
                .map(|name| format!("mega-{}", name))
                .collect::<Vec<_>>(),
            vec!["mega-droid", "mega-jaeger", "mega-mecha", "mega-robot"]
        );
    }
}
