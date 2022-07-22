
#[cfg(test)]
mod tests {
    use super::RRBscheduler;
    use super::super::{SchedulingPolicy,Task};
    #[test]
    fn test_sjf() {
        let mut rrb = RRBscheduler::new(2);

        let mut A = Task{
            id : "A",
            duration : 5,
            deadline : 10
        };

        let mut B = Task{
            id : "B",
            duration : 3,
            deadline : 9
        };

        let C = Task{
            id : "C",
            duration : 2,
            deadline : 8
        };

        let D = Task{
            id : "D",
            duration : 1,
            deadline : 12
        };

        rrb.register(A);
        A.duration = 2;
        assert_eq!(Some(A), rrb.take());

        rrb.register(B);
        rrb.register(C);
        rrb.register(D);
        assert_eq!(Some(A), rrb.take());
        B.duration = 2;
        assert_eq!(Some(B), rrb.take());
        assert_eq!(Some(C), rrb.take());
        assert_eq!(Some(D), rrb.take());
        A.duration = 1;
        assert_eq!(Some(A), rrb.take());
        B.duration = 1;
        assert_eq!(Some(B), rrb.take());
        assert_eq!(None, rrb.take());


    }


}

use crate::{Task, SchedulingPolicy};
use std::collections::VecDeque;

pub struct RRBscheduler {
    tasks: VecDeque<Task>,
    quantum : u32
}

impl RRBscheduler {
    pub fn new(quanta : u32) -> Self{
        Self {
            tasks: VecDeque::new(),
            quantum: quanta
        }
    }
}

impl SchedulingPolicy for RRBscheduler {

    fn register(&mut self, task: Task){
        self.tasks.push_back(task);
    }

    fn take(&mut self)-> Option<Task>{
        if let Some(mut task) = self.tasks.pop_front(){
            let mut t = task.clone();
            if task.duration >= self.quantum {
                t.duration = self.quantum;
                task.duration = task.duration-self.quantum;
                if task.duration > 0 {
                    self.tasks.push_back(task);
                }
            }else{
                t.duration = 1;
            }
            return Some(t);
        }
        return None;
    }
}


