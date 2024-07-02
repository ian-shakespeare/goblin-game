use std::cell::RefCell;

pub trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
    fn remove(&mut self, index: usize);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None);
    }

    fn remove(&mut self, index: usize) {
        let vec = self.get_mut();

        if index >= vec.len() {
            return;
        }

        if index == vec.len() - 1 {
            vec.pop();
            return;
        }

        let last_element = vec.pop().expect("Tried to pop element from empty vector");
        vec[index] = last_element;
    }
}
