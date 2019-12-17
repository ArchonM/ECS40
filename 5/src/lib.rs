#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
#[derive(Debug)]
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
    pub fn preorder(&self) -> IterPreorder<T> {
         IterPreorder::new(self)
    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> IterInorder<T> {
        IterInorder::new(self)
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> IterPostorder<T> {
        IterPostorder::new(self)
    }
}

pub struct IterPreorder<'a, T: 'a> {
    sta: Vec<&'a Tree<T>>,
}

impl<'a, T> IterPreorder<'a, T> {
    fn new(root: &'a Tree<T>) -> Self {
        let mut result = IterPreorder{sta: Vec::new()};
        result.sta.push(root);
        result
    }
}

impl<'a, T> Iterator for IterPreorder<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {

        if self.sta.len() != 0
        {
            let pre = self.sta.pop().unwrap();

            if let Some(ref right) = pre.right
            {
                self.sta.push(right);
            }

            if let Some(ref left) = pre.left
            {
                self.sta.push(left)
            }

            if let Some(ref val) = pre.key
            {
                return Some(val);
            }
            else
            {
                return None;
            }
        }
        else
        {
            return None;
        }
    }
}

pub struct IterInorder<'a, T: 'a> {
    sta: Vec<&'a Tree<T>>,
    cur: Vec<&'a Tree<T>>,
}

impl<'a, T> IterInorder<'a, T> {
    pub fn new(root: &'a Tree<T>) -> Self
    {
        let mut result = IterInorder
        {
            sta: Vec::new(),
            cur: Vec::new(),
        };
        result.cur.push(root);
        result
    }
}

impl<'a, T> Iterator for IterInorder<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>
    {
        if (self.cur.len() == 1) | (!self.sta.is_empty()) {
            while self.cur.len() == 1 {
                let temp = self.cur.pop().unwrap();
                match temp.left {
                    Some(ref left) => {self.cur.push(left);},
                    None => {},
                }
                self.sta.push(temp);
            } /*then*/{
                let left_most = self.sta.pop().unwrap();
                match left_most.right {
                    Some(ref right) => {self.cur.push(right);},
                    None => {},
                }
                match left_most.key
                {
                    Some(ref val) => return Some(val),
                    _ => return None,
                }
            }
        } else {
            None
        }


    }
}



pub struct IterPostorder<'a, T: 'a> {
    waiting: Vec<&'a Tree<T>>,
    pre: Option<&'a Tree<T>>,
}

impl<'a, T> IterPostorder<'a, T> {
    fn new(root: &'a Tree<T>) -> Self {
        let mut valid = IterPostorder{
            waiting: Vec::new(),
            pre: None,
        };
        let mut temp = root;
        valid.waiting.push(temp);
        loop {
            match temp.left {
                Some(ref left) => {
                    valid.waiting.push(left);
                    temp = left;
                },
                None => {break;},
            }
        }
        /*valid.waiting.push(temp);*/
        valid
    }

}

impl<'a, T> Iterator for IterPostorder<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cur = None::<&'a Tree<T>>;
        while (cur.is_some()) | (!self.waiting.is_empty()) {
            while cur.is_some() {
                let temp = cur.unwrap();
                self.waiting.push(temp);
                match temp.left {
                    Some(ref left) => {cur = Some(left);},
                    None => {cur = None;},
                }
            }
            cur = self.waiting.pop();
            let requirement: bool;
            if let Some(inside) = cur {
                match inside.right {
                    Some(ref right) => {
                        match self.pre {
                            Some(pre) => {
                                if right.key.as_ref().unwrap() as *const T== pre.key.as_ref().unwrap() as *const T
                                {
                                    requirement = true;
                                }
                                else
                                {
                                    requirement = false;
                                }
                            },
                            None => {requirement = false;},
                        }
                    },
                    None => {requirement = true;},
                }
                if requirement {
                    self.pre = cur;
                    match self.pre.unwrap().key
                    {
                        Some(ref val) => return Some(val),
                        _ => return None,
                    }
                } else {
                    self.waiting.push(inside);
                    match inside.right {
                        Some(ref right) => {cur = Some(right);},
                        None => {cur = None;},
                    }
                }
            }
        }
        None
    }
}
