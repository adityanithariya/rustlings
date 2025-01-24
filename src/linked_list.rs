use std::fmt::Display;

pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T> LinkedList<T>
where
    T: Display + Copy + PartialEq + PartialOrd,
{
    pub fn new(data: T, next: Option<Box<LinkedList<T>>>) -> LinkedList<T> {
        LinkedList { data, next }
    }

    pub fn get(&self, index: usize) -> &T {
        if index == 0 {
            &self.data
        } else {
            match self.next {
                Some(ref x) => x.get(index - 1),
                None => panic!("Index out of bounds!"),
            }
        }
    }
    pub fn next(&self) -> &Option<Box<LinkedList<T>>> {
        &self.next
    }
    pub fn print(&self) {
        print!("{}, ", self.data);
        match self.next {
            Some(ref x) => x.print(),
            None => println!(),
        }
    }
    pub fn insert(&mut self, data: T) {
        match self.next.as_mut() {
            Some(ref mut x) => x.insert(data),
            None => self.next = Some(Box::new(LinkedList { data, next: None })),
        }
    }
    pub fn insert_at(&mut self, data: T, index: usize) {
        if index == 0 {
            let next = self.next.take();
            self.next = Some(Box::new(LinkedList {
                data: self.data,
                next,
            }));
            self.data = data;
        } else if index == 1 {
            let next = self.next.take();
            self.next = Some(Box::new(LinkedList { data, next }));
        } else {
            match self.next.as_mut() {
                Some(ref mut x) => x.insert_at(data, index - 1),
                None => panic!("Index out of bounds!"),
            }
        }
    }
    pub fn delete(&mut self, index: usize) {
        if index == 0 {
            self.data = self.next.as_ref().unwrap().data;
            if let Some(x) = &mut self.next {
                self.next = x.next.take();
            }
        } else {
            match self.next.as_mut() {
                Some(ref mut x) => x.delete(index - 1),
                None => panic!("Index out of bounds!"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_create_linked_list() {
        let mut list = LinkedList::new(10, None);
        list.insert(20);
        list.insert_at(15, 0);
        list.print();
        assert_eq!(15, *list.get(0));
        assert_eq!(10, *list.get(1));
        assert_eq!(20, *list.get(2));
        list.delete(0);
        assert_eq!(10, *list.get(0));
    }
}
