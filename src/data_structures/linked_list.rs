use std::ptr;
use std::mem;

type Link<T> = Option<Box<Node<T>>>;

struct Raw<T> {
	ptr: *mut Node<T>
}

impl<T> Raw<T> {
	#[inline]
	fn none() -> Self {
		Raw {
			ptr: ptr::null_mut()
		}
	}

	#[inline]
	fn some(ptr: &mut Node<T>) -> Self {
		Raw { ptr }
	}

	#[inline]
	fn take(&mut self) -> Self {
		mem::replace(self, Raw::none())
	}

	#[inline]
	fn as_mut(&mut self) -> Option<&mut Node<T>> {
		if self.ptr.is_null() {
			None
		} else {
			unsafe {
				Some(&mut *self.ptr)
			}
		}
	}

	#[inline]
	fn as_ref(&self) -> Option<&Node<T>> {
		if self.ptr.is_null() {
			None
		} else {
			unsafe {
				Some(& *self.ptr)
			}
		}
	}
}

struct Node<T> {
	next: Link<T>,
	prev: Raw<T>,
	element: T,
}

impl<T> Node<T> {

	// Creates an empty Node.
	#[inline]
	fn new(element: T) -> Self {
		Node {
			next: None,
			prev: Raw::none(),
			element,
		}
	}

	// Creates a link between two nodes.
	#[inline]
	fn link(&mut self, mut next: Box<Self>) {
		next.prev = Raw::some(self);
		self.next = Some(next);
	}

	#[inline]
	fn take_next(&mut self) -> Option<Box<Self>> {
		let mut next = self.next.take();
		next.as_mut().map(|node| node.prev = Raw::none());
		next
	}
}

pub struct LinkedList<T> {
	head: Link<T>,
	tail: Raw<T>,
	length: usize,
}

// Private Methods
impl<T> LinkedList<T> {

	// Appends the new node at the end of the list.
	#[inline]
	fn push_back_node(&mut self, node: Node<T>) {
		let mut new_tail = Box::new(node);

		// Making the new node the new tail of the list and returning the old tail to old_tail (new_tail replaces the old_tail in memory)
		let mut old_tail = mem::replace(&mut self.tail, Raw::some(&mut *new_tail));

		match old_tail.as_mut() {
			// List is empty. So, the new node is also the new head.
			None => self.head = Some(new_tail),
			// List not empty. So, append new_tail next to old_tail. 
			Some(old_tail) => old_tail.link(new_tail)
		}
		self.length += 1;
	}

	// Appends the new node at the front of the list.
	#[inline]
	fn push_front_node(&mut self, node: Node<T>) {
		let mut new_head = Box::new(node);

		match self.head.take() {
			// List is empty. So, the new node is also the new tail.
			None => self.tail = Raw::some(&mut *new_head),
			// List not empty. So, append new_head before old_head.
			Some(old_head) => new_head.link(old_head)
		}		

		// Making the new node the head of the list.
		self.head = Some(new_head);
		self.length += 1;
	}

	// Removes the node at the back and returns the element in it.
	// Returns None if list is empty.
	#[inline]
	fn pop_back_node(&mut self) -> Option<T> {
		// Take() the current tail. If List is empty and_then returns None or else the inline function is called.
		self.tail.take().as_mut().and_then(|tail| {
			self.length -= 1;
			match tail.prev.take().as_mut() {
				// No Prev value for tail. So, list has only one node.
				// Remove the head and take the node.
				None => self.head.take().map(|node| node.element),
				// Tail has prev node. So, Make that node the new tail.
				// Take the node of the new tail's next field.
				Some(prev) => {
					self.tail = Raw::some(prev);
					prev.next.take().map(|node| node.element)
				}
			}			
		})
	}

	// Removes the node at the front and returns the element in it.
	// Returns None if list is empty.	
	#[inline]
	fn pop_front_node(&mut self) -> Option<T> {
		self.head.take().map(|mut head| {
			self.length -= 1;
			match head.take_next() {
				// No Next value for head.
				None => self.tail = Raw::none(),
				// Head has next value. Making that the new head.
				Some(new_head) => self.head = Some(new_head),
			}
			head.element
		})
	}
}

// Public Methods
impl<T> LinkedList<T> {
	pub fn new() -> Self {
		LinkedList {
			head: None,
			tail: Raw::none(),
			length: 0
		}
	}

	/// Inserts the element at the end of the List.
	pub fn push(&mut self, element: T) {
		self.push_back(element);
	}

	/// Removes the element from the end of the List and returns it.
	///
	/// Returns `None` if List is empty.
	pub fn pop(&mut self) -> Option<T> {
		self.pop_back()
	}

	/// Inserts the element at the end of the List.
	pub fn push_back(&mut self, element: T) {
		self.push_back_node(Node::new(element));
	}

	/// Removes the element from the end of the List and returns it.
	///
	/// Returns `None` if List is empty.
	pub fn pop_back(&mut self) -> Option<T>{
		self.pop_back_node()
	}

	/// Inserts the element at the front of the List.
	pub fn push_front(&mut self, element: T) {
		self.push_front_node(Node::new(element));
	}

	/// Removes the element from the front of the List and returns it.
	///
	/// Returns `None` if List is empty.
	pub fn pop_front(&mut self) -> Option<T> {
		self.pop_front_node()
	}

	pub fn insert() {

	}

	/// Returns the no. of elements in the List.
	pub fn length(&self) -> usize {
		self.length
	}

	/// Returns if the List is empty or not.
	pub fn is_empty(&self) -> bool {
		self.length == 0
	}

	pub fn contains() {

	}

	/// Removes all the elements from the List.
	pub fn clear(&mut self) {
		while !self.is_empty() {
			self.pop();
		}
	}

	/// Returns a reference to the element at the front of the List.
	///
	/// Returns `None` if the List is empty.
	pub fn front(&self) -> Option<&T> {
		self.head.as_ref().map(|node| &node.element)
	}

	/// Returns a reference to the element at the back of the List.
	///
	/// Returns `None` if the List is empty.
	pub fn back(&self) -> Option<&T> {
		self.tail.as_ref().map(|node| &node.element)
	}

	/// Returns a mutable reference to the element at the front of the List.
	///
	/// Returns `None` if the List is empty.
	pub fn front_mut(&mut self) -> Option<&mut T> {
		self.head.as_mut().map(|node| &mut node.element)
	}

	/// Returns a mutable reference to the element at the back of the List.
	///
	/// Returns `None` if the List is empty.
	pub fn back_mut(&mut self) -> Option<&mut T> {
		self.tail.as_mut().map(|node| &mut node.element)
	}
//	Some other Methods
//
//	Sort
//	Merge Two Lists
// 	Search
//	Reverse
//	Rotate
//	Get(index)
//	Delete at index
}