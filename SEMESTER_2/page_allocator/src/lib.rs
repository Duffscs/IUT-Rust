#[cfg(test)]
mod tests {
    use crate::PageAllocator;

    #[test]
    fn check_capacity() {
        let _capactiy = 3;
        let a = PageAllocator::new(_capactiy);
        assert_eq!(a.page_operator.capacity(), _capactiy);
    }

    #[test]
    fn check_allocation_zero() {
        let _capactiy = 3;
        let mut a = PageAllocator::new(_capactiy);
        assert_eq!(a.allocate(),Some(0));
        assert_eq!(a.page_operator[0], true);
    }

    #[test]
    fn check_allocation_un() {
        let capactiy = 3;
        let mut a = PageAllocator::new(capactiy);
        a.allocate();
        assert_eq!(a.allocate(),Some(1));
    }

    #[test]
    fn check_allocation_deux() {
        let _capactiy = 3;
        let mut a = PageAllocator::new(_capactiy);
        a.allocate();
        a.allocate();
        assert_eq!(a.allocate(),Some(2));
    }

    #[test]
    fn check_allocation_plein() {
        let _capactiy = 3;
        let mut a = PageAllocator::new(_capactiy);
        a.allocate();
        a.allocate();
        a.allocate();
        assert_eq!(a.allocate(),None);
    }

    #[test]
    fn check_free() {
        let _capactiy = 3;
        let mut a = PageAllocator::new(10);
        assert_eq!(a.allocate(),Some(0));
        a.free(0);
        assert_eq!(a.page_operator[0], false);

    }

    #[test]
    fn to_string_vide() {
        let _capactiy = 3;
        let a = PageAllocator::new(3);
        assert_eq!(a.to_string(),String::from("---"));
    }

    #[test]
    fn to_string_un() {
        let _capactiy = 3;
        let mut a = PageAllocator::new(3);
        a.allocate();
        assert_eq!(a.to_string(),String::from("x--"));
    }

    #[test]
    fn to_string_deux() {
        let _capactiy = 3;
        let mut a = PageAllocator::new(3);
        a.allocate();
        a.allocate();
        assert_eq!(a.to_string(),String::from("xx-"));
    }

    #[test]
    fn to_string_trois() {
        let _capactiy = 3;
        let mut a = PageAllocator::new(3);
        a.allocate();
        a.allocate();
        a.allocate();
        assert_eq!(a.to_string(),String::from("xxx"));
    }
}

use std::collections::VecDeque;

struct PageAllocator {
   page_operator: Vec<bool>,
   position_libre: VecDeque<usize>
}



impl PageAllocator {
    pub fn new(capacity : usize) -> Self{
        Self {
            page_operator: vec![false; capacity],
            position_libre: (0..capacity).collect()
        }
    }

    pub fn allocate(&mut self) -> Option<usize>{
        if let Some(pos_libre) = self.position_libre.pop_front(){
            self.page_operator[pos_libre] = true;
            return Some(pos_libre);
        }
        return None;
    }

    pub fn free(&mut self, page : usize){
        self.page_operator[page] = false;
        self.position_libre.push_back(page);
    }

    fn to_string(&self) -> String{
        let mut str= String::new();
        for i in self.page_operator.iter(){
            if *i{
                str.push('x');
            }else {
                str.push('-');
            }
        }
        return str;
    }
}
