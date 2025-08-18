pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {

    /// Creates a new `Node` with a given value and next option.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be stored in the `Node`.
    /// * `next` - The next `Node` in the linked list, if any.
    ///
    fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T>{
        Node { value, next }
    }
}

impl<T> Default for LinkedList<T> {

    /// Returns a new `LinkedList` with a `head` set to `None`.
    fn default() -> Self {
        LinkedList { head: None }
    }
}


impl<T> LinkedList<T> {

    /// Creates a new LinkedList with an optional collection of items.
    ///
    /// # Arguments
    ///
    /// * `Collection` - An optional collection of items of type `T`.
    ///
    /// # Examples
    ///
    /// 
    pub fn from<I>(Collection: Option<Input<T, I>>) -> LinkedList<T>
    where 
        I: IntoIterator<Item = T>,
     {

        let mut new_list = LinkedList {
            head: None,
        };

        if let Some(input) = Collection {
            new_list.push(input);
        }
        new_list
    }

    /// Creates a new `LinkedList` with a `head` set to `None`.
    ///
    pub fn new() -> LinkedList<T> {
        LinkedList::default()
    }

}


pub enum Input<T, C>
where
    C: IntoIterator<Item = T>,
{
    Item(T),
    Collection(C),
}


impl<T> LinkedList<T> {
    /// Pushes a collection of items into the linked list. 
    ///
    /// Takes an `Input` enum, which can be either a single item or a collection of items.
    /// Each item is then added to the start of the linked list.
    ///
    pub fn push<I>(&mut self, Collection: Input<T, I>)
    where
        I: IntoIterator<Item = T>
    {
        let collection_vec: Vec<T> = match Collection {
            Input::Collection(c) => c.into_iter().collect(),
            Input::Item(item) => vec![item],
        };

        for value in collection_vec.into_iter().rev() {
            let new_node = Box::new(Node {
                value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        }
    }   
}

