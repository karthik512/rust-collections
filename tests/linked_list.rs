use collections::data_structures::LinkedList;

#[test]
fn linked_list_insertion() {
	let mut list: LinkedList<i32> = LinkedList::new();

	assert_eq!(true, list.is_empty());
	assert_eq!(0, list.length());

	list.push_back(0); list.push_back(1);

	assert_eq!(false, list.is_empty());
	assert_eq!(2, list.length());

	list.pop();
	assert_eq!(1, list.length());

	list.clear();
	assert_eq!(0, list.length());
}

#[test]
fn into_iter() {
	let mut list: LinkedList<i32> = LinkedList::new();

	list.push_back(10); list.push_back(20); list.push_back(30);

	let mut iter = list.into_iter();
	assert_eq!(iter.next(), Some(10));
	assert_eq!(iter.next(), Some(20));
	assert_eq!(iter.next(), Some(30));
	assert_eq!(iter.next(), None);
}

#[test]
fn into_iter_back() {
	let mut list: LinkedList<i32> = LinkedList::new();

	list.push_back(10); list.push_back(20); list.push_back(30);

	let mut iter = list.into_iter();
	assert_eq!(iter.next_back(), Some(30));
	assert_eq!(iter.next_back(), Some(20));
	assert_eq!(iter.next_back(), Some(10));
	assert_eq!(iter.next_back(), None);
}

#[test]
fn iter() {
	let mut list: LinkedList<i32> = LinkedList::new();

	let mut iter = list.iter();
	assert_eq!(iter.next(), None);

	list.push(10); list.push(20); list.push(30);

	let mut iter = list.iter();
	assert_eq!(iter.next(), Some(&10));
	assert_eq!(iter.next(), Some(&20));
	assert_eq!(iter.next(), Some(&30));
	assert_eq!(iter.next(), None);
}

#[test]
fn iter_back() {
	let mut list: LinkedList<i32> = LinkedList::new();

	let mut iter = list.iter();
	assert_eq!(iter.next_back(), None);

	list.push(10); list.push(20); list.push(30);

	let mut iter = list.iter();
	assert_eq!(iter.next_back(), Some(&30));
	assert_eq!(iter.next_back(), Some(&20));
	assert_eq!(iter.next_back(), Some(&10));
	assert_eq!(iter.next_back(), None);
}