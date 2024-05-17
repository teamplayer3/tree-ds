use std::cell::RefCell;
use std::fmt::Display;
#[cfg(not(feature = "async"))]
use std::rc::Rc;
#[cfg(feature = "async")]
use std::sync::Arc;

/// A node in a tree.
///
/// This struct represents a node in a tree. The node has a unique id, a value, children and a parent. The unique id
/// is used to identify the node. The value is the value of the node. The children are the children of the node and
/// the parent is the parent of the node.
///
/// # Type Parameters
///
/// * `Q` - The type of the unique id of the node. Odd, I know but this is for flexibility. Some people might want to use
/// a string as the unique id of the node. Others might want to use an integer. This is why the unique id is a generic type.
/// * `T` - The type of the value of the node.
///
/// # Fields
///
/// * `node_id` - The unique id of the node.
/// * `value` - The value of the node.
/// * `children` - The children of the node.
/// * `parent` - The parent of the node.
///
/// # Example
///
/// ```rust
/// # use tree_ds::prelude::Node;
///
/// let node: Node<i32, i32> = Node::new(1, Some(2));
/// ```
#[cfg(not(feature = "async"))]
#[derive(Clone, Debug, Eq)]
pub struct Node<Q, T>(Rc<RefCell<_Node<Q, T>>>) where Q: PartialEq + Eq + Clone, T: PartialEq + Eq + Clone;

/// A node in a tree.
///
/// This struct represents a node in a tree. The node has a unique id, a value, children and a parent. The unique id
/// is used to identify the node. The value is the value of the node. The children are the children of the node and
/// the parent is the parent of the node.
///
/// # Type Parameters
///
/// * `Q` - The type of the unique id of the node. Odd, I know but this is for flexibility. Some people might want to use
/// a string as the unique id of the node. Others might want to use an integer. This is why the unique id is a generic type.
/// * `T` - The type of the value of the node.
///
/// # Fields
///
/// * `node_id` - The unique id of the node.
/// * `value` - The value of the node.
/// * `children` - The children of the node.
/// * `parent` - The parent of the node.
///
/// # Example
///
/// ```rust
/// # use tree_ds::prelude::Node;
///
/// let node: Node<i32, i32> = Node::new(1, Some(2));
/// ```
#[cfg(feature = "async")]
#[derive(Clone, Debug, Eq)]
pub struct Node<Q, T>(Arc<RefCell<_Node<Q, T>>>) where Q: PartialEq + Eq + Clone, T: PartialEq + Eq + Clone;

impl<Q, T> Node<Q, T> where Q: PartialEq + Eq + Clone, T: PartialEq + Eq + Clone {
	/// Create a new node.
	///
	/// This method creates a new node with the given node id and value. The node id is used to identify the node
	/// and the value is the value of the node. The value can be used to store any data that you want to associate
	/// with the node.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node.
	/// * `value` - The value of the node.
	///
	/// # Returns
	///
	/// A new node with the given node id and value.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::Node;
	///
	/// let node = Node::new(1, Some(2));
	/// ```
	#[cfg(not(feature = "async"))]
	pub fn new(node_id: Q, value: Option<T>) -> Self {
		Node(Rc::new(RefCell::new(_Node {
			node_id,
			value,
			children: vec![],
			parent: None,
		})))
	}

	/// Create a new node.
	///
	/// This method creates a new node with the given node id and value. The node id is used to identify the node
	/// and the value is the value of the node. The value can be used to store any data that you want to associate
	/// with the node.
	///
	/// # Arguments
	///
	/// * `node_id` - The id of the node.
	/// * `value` - The value of the node.
	///
	/// # Returns
	///
	/// A new node with the given node id and value.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::Node;
	///
	/// let node = Node::new(1, Some(2));
	/// ```
	#[cfg(feature = "async")]
	pub fn new(node_id: Q, value: Option<T>) -> Self {
		Node(Arc::new(RefCell::new(_Node {
			node_id,
			value,
			children: vec![],
			parent: None,
		})))
	}

	/// Add a child to the node.
	///
	/// This method adds a child to the node. The child is added to the children of the node and the parent
	/// of the child is set to the node.
	///
	/// # Arguments
	///
	/// * `child` - The child to add to the node.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::Node;
	///
	/// let parent_node = Node::new(1, Some(2));
	/// parent_node.add_child(Node::new(2, Some(3)));
	/// ```
	pub fn add_child(&self, child: Node<Q, T>) {
		let mut node = self.0.borrow_mut();
		node.children.push(child.clone());
		let mut child = child.0.borrow_mut();
		child.parent = Some(self.clone());
	}

	/// Get the unique Id of the node.
	///
	/// This method returns the unique Id of the node. The unique Id is used to identify the node.
	///
	/// # Returns
	///
	/// The unique Id of the node.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::Node;
	///
	/// let node = Node::new(1, Some(2));
	/// assert_eq!(node.get_node_id(), 1);
	/// ```
	pub fn get_node_id(&self) -> Q {
		self.0.borrow().node_id.clone()
	}

	/// Get the children of the node.
	///
	/// This method returns the children of the node.
	///
	/// # Returns
	///
	/// The children of the node.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::Node;
	///
	/// let node = Node::new(1, Some(2));
	/// let child = Node::new(2, Some(3));
	/// node.add_child(child);
	/// assert_eq!(node.get_children().len(), 1);
	/// ```
	pub fn get_children(&self) -> Vec<Node<Q, T>> {
		self.0.borrow().children.clone()
	}

	/// Get the parent of the node.
	///
	/// This method returns the parent of the node. In the case where the node is a root node in a tree,
	/// the parent of the node will be `None`.
	///
	/// # Returns
	///
	/// The optional parent of the node.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::Node;
	///
	/// let parent_node = Node::new(1, Some(2));
	/// let child_node = Node::new(2, Some(3));
	/// parent_node.add_child(child_node.clone());
	/// assert_eq!(child_node.get_parent().as_ref(), Some(&parent_node));
	/// assert!(parent_node.get_parent().is_none());
	/// ```
	pub fn get_parent(&self) -> Option<Node<Q, T>> {
		self.0.borrow().parent.clone()
	}

	/// Get the value of the node.
	///
	/// This method returns the value of the node. If the node has no value, `None` is returned.
	///
	/// # Returns
	///
	/// The value of the node.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::Node;
	///
	/// let node = Node::new(1, Some(2));
	/// assert_eq!(node.get_value(), Some(2));
	/// ```
	pub fn get_value(&self) -> Option<T> {
		self.0.borrow().value.clone()
	}

	/// Set the value of the node.
	///
	/// This method sets the value of the node.
	///
	/// # Arguments
	///
	/// * `value` - The value to set.
	///
	/// # Example
	///
	/// ```rust
	/// # use tree_ds::prelude::Node;
	///
	/// let node = Node::new(1, Some(2));
	/// assert_eq!(node.get_value(), Some(2));
	/// node.set_value(Some(3));
	/// assert_eq!(node.get_value(), Some(3));
	/// ```
	pub fn set_value(&self, value: Option<T>) {
		self.0.borrow_mut().value = value;
	}
}

impl<Q, T> PartialEq for Node<Q, T> where Q: PartialEq + Eq + Clone, T: PartialEq + Eq + Clone {
	fn eq(&self, other: &Self) -> bool {
		self.get_node_id() == other.get_node_id() && self.get_value() == other.get_value()
	}
}

impl<Q, T> Display for Node<Q, T> where Q: PartialEq + Eq + Clone + Display, T: PartialEq + Eq + Clone + Display {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Node {{ Id: {}, Value: {} }}", self.get_node_id(), self.get_value().as_ref().unwrap())
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct _Node<Q, T> where Q: PartialEq + Eq + Clone, T: PartialEq + Eq + Clone {
	/// The user supplied id of the node.
	node_id: Q,
	/// The value of the node.
	value: Option<T>,
	/// The children of the node.
	children: Vec<Node<Q, T>>,
	/// The parent of the node.
	parent: Option<Node<Q, T>>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_node() {
		let node = Node::new(1, Some(2));
		let child = Node::new(2, Some(3));
		node.add_child(child);
		assert_eq!(node.get_children().len(), 1);
	}

	#[test]
	fn test_node_get_node_id() {
		let node = Node::new(1, Some(2));
		assert_eq!(node.get_node_id(), 1);
	}

	#[test]
	fn test_node_get_parent() {
		let parent_node = Node::new(1, Some(2));
		let child_node = Node::new(2, Some(3));
		parent_node.add_child(child_node.clone());
		assert_eq!(child_node.get_parent().as_ref(), Some(&parent_node));
		assert!(parent_node.get_parent().is_none());
	}

	#[test]
	fn test_node_get_value() {
		let node = Node::new(1, Some(2));
		assert_eq!(node.get_value(), Some(2));
	}

	#[test]
	fn test_node_set_value() {
		let node = Node::new(1, Some(2));
		assert_eq!(node.get_value(), Some(2));
		node.set_value(Some(3));
		assert_eq!(node.get_value(), Some(3));
	}
}
