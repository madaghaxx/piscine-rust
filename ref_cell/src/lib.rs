use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: RefCell<u32>,
    pub max: u32,
}

impl Tracker {
    pub fn new(max: u32) -> Self {
        Tracker {
            messages: RefCell::new(Vec::new()),
            value: RefCell::new(0),
            max,
        }
    }

    pub fn set_value<T>(&self, rc: &Rc<T>) {
        let ref_count = Rc::strong_count(rc) as u32;
        
        if ref_count > self.max {
            self.messages.borrow_mut().push("Error: You can't go over your quota!".to_string());
        } else {
            *self.value.borrow_mut() = ref_count;
            let percentage = (ref_count * 100) / self.max;
            
            if percentage > 70 {
                self.messages.borrow_mut().push(format!("Warning: You have used up over {}% of your quota!", percentage));
            }
        }
    }

    pub fn peek<T>(&self, rc: &Rc<T>) {
        let ref_count = Rc::strong_count(rc) as u32;
        let percentage = (ref_count * 100) / self.max;
        
        self.messages.borrow_mut().push(format!("Info: This value would use {}% of your quota", percentage));
    }
}
