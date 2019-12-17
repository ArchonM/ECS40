#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

pub struct Tree<T> {
    key: Option<T>,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree{
            key: None,
            left: None,
            right: None,
        }
    }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {
        match self.key {
            Some(ref val) => {
                if *val < key {
                    if let Some(ref mut right) = self.right {
                        right.insert(key);
                    }
                    else {
                        self.right = Some(Box::new(Tree{key:Some(key),left:None,right:None}));
                        return true;
                    }
                }
                else if *val > key {
                    if let Some(ref mut left) = self.left {
                        left.insert(key);
                    }
                    else {
                        self.left = Some(Box::new(Tree{key:Some(key),left:None,right:None}));
                    }
                    return true;
                }
            }
            None => {
                self.key = Some(key);
                return true;
                }

        }
        false
    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.key {
            Some(ref val) =>
            {
                if val == key {
                    return true;
                }
                else if val < key {
                    if let Some(ref right) = self.right {
                        return right.find(key);
                    }
                    else {
                        return false;
                    }
                }
                else {
                    if let Some(ref left) = self.left {
                        return left.find(key);
                    }
                    else {
                        return false;
                    }
                }
            }
            _ => false,
        }
    }

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        let mut result:Vec<&T> = Vec::new();
        let mut temp:Vec<&T>;
        match self.key
        {
            Some(ref val) => result.push(val),
            _ => panic!(),
        }

        if let Some(ref left) = self.left {
            temp = left.preorder();
            for v in temp {
                result.push(v);
            }
        }

        if let Some(ref right) = self.right {
            temp = right.preorder();
            for v in temp {
                result.push(v);
            }
        }

        result
    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut result:Vec<&T> = Vec::new();
        let mut temp:Vec<&T>;


        if let Some(ref left) = self.left {
            temp = left.inorder();
            for v in temp {
                result.push(v);
            }
        }

        match self.key
        {
            Some(ref val) => result.push(val),
            _ => panic!(),
        }

        if let Some(ref right) = self.right {
            temp = right.inorder();
            for v in temp {
                result.push(v);
            }
        }

        result
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut result:Vec<&T> = Vec::new();
        let mut temp:Vec<&T>;

        if let Some(ref left) = self.left {
            temp = left.postorder();
            for v in temp {
                result.push(v);
            }
        }

        if let Some(ref right) = self.right {
            temp = right.postorder();
            for v in temp {
                result.push(v);
            }
        }

        match self.key
        {
            Some(ref val) => result.push(val),
            _ => panic!(),
        }

        result
    }
}
