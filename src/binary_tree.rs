


pub struct BinaryTree<'a, T> {
    root: BinaryTreeLink<'a, T>,
}

struct TreeNode<'a, T> {
        left_child: BinaryTreeLink<'a, T>,
        right_child: BinaryTreeLink<'a, T>,
        data : &'a T,
}

type BinaryTreeLink<'a, T> = Option<Box<TreeNode<'a, T>>>;


impl <'a, T> TreeNode<'a, T>
    where T: Ord
{
    pub fn new(data: &'a T) -> Self {
        TreeNode{
            left_child: None,
            right_child: None,
            data: data
        }
    }

    pub fn insert_data(&mut self, new_data: &'a T) {
        if self.data == new_data {
            return;
        }
        let next = if self.data < new_data {
            &mut self.right_child
        } else {&mut self.left_child};
        match next {
            &mut Some(ref mut n) => n.insert_data(new_data),
            _ => {
                let new_tree_node = TreeNode{left_child: None,
                    right_child: None,
                    data: new_data,
                };
                *next = Some(Box::new(new_tree_node));
            }
        }
    }

    pub fn inorder_traversal(&self, list: &mut Vec<&'a T>) {
        match &self.left_child {
            Some(child) => child.inorder_traversal(list),
            _ => {}
        }
        list.push(self.data);
        match &self.right_child {
            Some(child) => child.inorder_traversal(list),
            _ => {}
        }
    }

}

impl <'a, T> BinaryTree<'a, T>
    where T: Ord
{
    pub fn new() -> Self {
        BinaryTree{
            root: None,
        }
    }

    pub fn insert_data(&mut self, new_data: &'a T) {
        match self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(new_data)));
                return;
            },
            Some(ref mut tree) => {tree.insert_data(new_data);}
        }
    }

    pub fn insert_list(&mut self, new_data_list: &'a [T]) {
        for a in new_data_list {
            self.insert_data(&a);
        }
    }

    pub fn create_list(&self) -> Vec<&'a T> {
        let mut ret: Vec<&'a T> = vec![];
        match &self.root {
            None => {},
            Some(root) => root.inorder_traversal(&mut ret),
        }
        return ret;
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

    #[test]
    fn insert_data_and_to_list() {
        let mut tree: BinaryTree<u32> = BinaryTree::new();
        let a1 = 1;
        let a2 = 2;
        let a3 = 3;
        let a4 = 4;
        let a5 = 5;
        tree.insert_data(&a1);
        tree.insert_data(&a2);
        tree.insert_data(&a3);
        tree.insert_data(&a4);
        tree.insert_data(&a5);
        let res = tree.create_list();
        assert_eq!(res, [&a1, &a2, &a3,&a4, &a5 ])

    }

}
