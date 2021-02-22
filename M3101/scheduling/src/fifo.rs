#[cfg(test)]
mod tests {
    use super::FifoScheduler;
    use super::super::{SchedulingPolicy,Task};
    #[test]
    fn test_fifo() {
        let mut fifo = FifoScheduler::new();

        let A = Task{
            id : "A",
            duration : 5,
            deadline : 10
        };

        let B = Task{
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

        fifo.register(A);

        assert_eq!(Some(A),fifo.take());

        fifo.register(B);
        fifo.register(C);
        fifo.register(D);

        assert_eq!(Some(B),fifo.take());
        assert_eq!(Some(C),fifo.take());
        assert_eq!(Some(D),fifo.take());
        assert_eq!(None,fifo.take());


    }


}

use std::collections::VecDeque;
use crate::{Task, SchedulingPolicy};

pub struct FifoScheduler {
    tasks: VecDeque<Task>
}

impl FifoScheduler {
    pub fn new() -> Self{
        Self{
            tasks: VecDeque::new(),
        }
    }
}

impl SchedulingPolicy for FifoScheduler {

    fn register(&mut self, task: Task){
        self.tasks.push_back(task);
    }

    fn take(&mut self)-> Option<Task>{
        return self.tasks.pop_front();
    }

}
