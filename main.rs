//Reference: https://en.wikipedia.org/wiki/Red%E2%80%93black_tree
//the exact same code can be found in wikipedia in C
//todo: remove(node)

//red-black tree properties
//1) every node is either red or red-black.
//2) all null nodes are consider black.
//3) a red node does not have a red child. (aka red-violation)
//4) every path from a given node to any nodes goes through the same number of black nodes.
//5) if a node N has exactly one child, the child must be red. If the child were black, its
//   leaves would sit at different black depth than N's null node

//lets explain the point 4 more visually
/*
 *      r: red
 *      b: black
 *      n: null(is considered black)
 *
 *             [r](root node)
 *            /   \
 *           b     b
 *          / \   / \
 *         n   r n   n
 *            /
 *           n
 *  as you can see that from root node 
 *      [r](red) to all paths:
 *      
 *      (1) [r] -> b -> n = total 2 black nodes 
 *      (2) [r] -> b -> r -> n = total 2 black nodes      
 *      (3) [r] -> b -> n = total 2 black nodes
 *      (4) [r] -> b -> n = total 2 black nodes
 *          
 */

//the code is written in unsafe mode more or less in C-style
use std::ptr::null_mut;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Color {
    Red,
    Black,
}

#[derive(Clone, Copy)]
enum Dir {
    Left = 0,
    Right = 1,
}

impl Dir {
    fn opposite(self) -> Self {
        match self {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

struct Info {
    child: [*mut Node; 2],
}

struct Node {
    key: i32,
    color: Color,
    parent: *mut Node,
    info: Info,
}

struct Tree {
    root: *mut Node,
}

impl Tree {
    fn new() -> Self {
        Tree { root: null_mut() }
    }

    //creates a node
    unsafe fn create_node(key: i32) -> *mut Node {
        let node = Box::into_raw(Box::new(Node {
            key,
            color: Color::Red,
            parent: null_mut(),
            info: Info { child: [null_mut(), null_mut()] },
        }));
        node
    }

    fn direction(node: *mut Node) -> Dir {
        unsafe {
            let parent = (*node).parent;
            if parent.is_null() {
                panic!("Node has no parent");
            }
            if (*parent).info.child[0] == node {
                Dir::Left
            } else {
                Dir::Right
            }
        }
    }

    //rotates the subtree
    unsafe fn rotate_subtree(&mut self, sub: *mut Node, dir: Dir) {
        unsafe {
            let opp_dir = dir.opposite();
            let new_root = (*sub).info.child[opp_dir as usize];

            if new_root.is_null() {
                return;
            }

            let new_child = (*new_root).info.child[dir as usize];
            (*sub).info.child[opp_dir as usize] = new_child;
            if !new_child.is_null() {
                (*new_child).parent = sub;
            }

            (*new_root).parent = (*sub).parent;
            if (*sub).parent.is_null() {
                self.root = new_root;
            } else {
                let parent = (*sub).parent;
                let parent_dir = if (*parent).info.child[0] == sub { 0 } else { 1 };
                (*parent).info.child[parent_dir] = new_root;
            }

            (*new_root).info.child[dir as usize] = sub;
            (*sub).parent = new_root;
        }
    }

    //after every insert() we have to check if the tree is unbalanced 
    //which usually happens in BST and in every insert_fix() checks 
    //the rules and if any of them are violated then it does its respective case operations.
    unsafe fn insert_fix(&mut self, mut node: *mut Node) {
        unsafe {
            let mut parent = (*node).parent;

            while !parent.is_null() && (*parent).color == Color::Red {
                let grandparent = (*parent).parent;
                let dir = Tree::direction(parent);
                let uncle = (*grandparent).info.child[dir.opposite() as usize];

                if uncle.is_null() || (*uncle).color == Color::Black {
                    if node == (*parent).info.child[dir.opposite() as usize] {
                        node = parent;
                        parent = (*node).parent;
                        self.rotate_subtree(parent, dir);
                    }

                    parent = (*node).parent;
                    let grandparent = (*parent).parent;

                    (*parent).color = Color::Black;
                    (*grandparent).color = Color::Red;
                    self.rotate_subtree(grandparent, dir.opposite());
                } else {
                    (*parent).color = Color::Black;
                    (*uncle).color = Color::Black;
                    (*grandparent).color = Color::Red;

                    node = grandparent;
                    parent = (*node).parent;
                }
            }

            if !self.root.is_null() {
                (*self.root).color = Color::Black;
            }
        }
    }

    fn insert(&mut self, key: i32) {
        unsafe {
            let node = Tree::create_node(key);

            if self.root.is_null() {
                self.root = node;
                (*node).color = Color::Black;
                return;
            }

            let mut curr = self.root;
            let mut parent = null_mut();
            let mut dir = Dir::Left;

            //reguler bst insertion
            while !curr.is_null() {
                parent = curr;
                dir = if key < (*curr).key { Dir::Left } else { Dir::Right };
                curr = (*curr).info.child[dir as usize];
            }

            (*node).parent = parent;
            (*parent).info.child[dir as usize] = node;

            self.insert_fix(node);
        }
    }

    fn inorder(&self, node: *mut Node) {
        unsafe {
            if node.is_null() {
                return;
            }

            self.inorder((*node).info.child[0]);
            println!(
                "{} ({})",
                (*node).key,
                if (*node).color == Color::Red { "R" } else { "B" }
            );
            self.inorder((*node).info.child[1]);
        }
    }

    fn print_inorder(&self) {
        println!("In-order Traversal:");
        self.inorder(self.root);
    }
}

fn main() {
    let mut tree = Tree::new();

    let keys = vec![20, 15, 25, 10, 5, 30];
    for key in keys {
        tree.insert(key);
    }

    tree.print_inorder();
}

