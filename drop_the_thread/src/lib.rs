use std::cell::{ Cell, RefCell };

#[derive(Debug)]
pub struct ThreadPool {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl ThreadPool {
    pub fn new() -> Self {
        Self {
            drops: Cell::new(0),
            states: RefCell::new(Vec::new()),
        }
    }

    pub fn new_thread(&self, cmd: String) -> (usize, Thread<'_>) {
        let mut states = self.states.borrow_mut();
        let id = states.len();
        states.push(false);
        (id, Thread { pid: id, cmd, parent: self })
    }

    pub fn thread_len(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow()[id]
    }

    pub fn drop_thread(&self, id: usize) {
        if id >= self.states.borrow().len() {
            panic!("{} is already dropped", id);
        }

        let mut states = self.states.borrow_mut();
        if states[id] {
            panic!("{} is already dropped", id);
        }

        states[id] = true;
        self.drops.set(self.drops.get() + 1);
    }
}
#[derive(Debug)]
pub struct Thread<'a> {
    pub pid: usize,
    pub cmd: String,
    pub parent: &'a ThreadPool,
}

impl Thread<'_> {
    pub fn skill(self) {
        drop(self);
    }
}

impl Drop for Thread<'_> {
    fn drop(&mut self) {
        self.parent.drop_thread(self.pid);
    }
}
