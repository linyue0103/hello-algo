/**
 * File: binary_tree.rs
 * Created Time: 2023-02-27
 * Author: xBLACKICEx (xBLACKICE@outlook.com)
 */

use std::rc::Rc;
include!("../include/include.rs");
use tree_node::TreeNode;

/* Driver Code */
fn main() {
    // 初始化二叉树
    let n1 = TreeNode::new(1);
    let n2 = TreeNode::new(2);
    let n3 = TreeNode::new(3);
    let n4 = TreeNode::new(4);
    let n5 = TreeNode::new(5);
    // 构建引用指向（即指针）
    n1.borrow_mut().left = Some(Rc::clone(&n2));
    n1.borrow_mut().right = Some(Rc::clone(&n3));
    n2.borrow_mut().left = Some(Rc::clone(&n4));
    n2.borrow_mut().right = Some(Rc::clone(&n5));
    println!("\n初始化二叉树\n");
    print_util::print_tree(&n1);

    // 插入结点与删除结点
    let p = TreeNode::new(0);
    // 在 n1 -> n2 中间插入结点 P
    p.borrow_mut().left = Some(Rc::clone(&n2));
    n1.borrow_mut().left = Some(Rc::clone(&p));
    println!("\n插入结点 P 后\n");
    print_util::print_tree(&n1);
    // 删除结点 P
    drop(p);
    n1.borrow_mut().left = Some(Rc::clone(&n2));
    println!("\n删除结点 P 后\n");
    print_util::print_tree(&n1);
}