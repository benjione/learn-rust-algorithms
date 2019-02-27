


pub struct BinaryTree<T> {
    root: BinaryTreeLink<T>,
}

struct TreeNode<T> {
        left_child: BinaryTreeLink<T>,
        right_child: BinaryTreeLink<T>,
        data : T,
}

type BinaryTreeLink<T> = Option<Box<TreeNode<T>>>;


impl <T> TreeNode<T> {
    pub fn new(data: T) -> Self {
        TreeNode{
            left_child: None,
            right_child: None,
            data: data
        }
    }
}

impl <T> BinaryTree<T>
    where T: Ord
{
    pub fn new() -> Self {
        BinaryTree{
            root: None,
        }
    }

    pub fn insert_data(&mut self, new_data: T) {
        match self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(new_data)));
                return;
            },
            _ => {}
        }
        let mut ele = &mut self.root;
        let mut pre_ele: &mut BinaryTreeLink<T> = &mut None;
        let mut new_ele:  &mut BinaryTreeLink<T> = &mut None;
        loop {
            match ele {
                None => {return;},
                Some(tree) => {
                    if tree.data > new_data {
                        new_ele = &mut tree.left_child;
                    }
                    else if tree.data < new_data {
                         new_ele = &mut tree.right_child;
                    }
                }
            }
            // pre_ele = ele;
            // ele = new_ele;
        }
    }



}


#[cfg(test)]
mod tests {
    use super::BinaryTree;
    #[test]
    fn test_new_tree() {
        let tree: BinaryTree<u32> = BinaryTree::new();
        //TODO: make a compare test
    }

}
