
#[cfg(test)]
mod tests {
    use super::SJFscheduler;
    use super::super::{SchedulingPolicy,Task};
    #[test]
    fn test_sjf() {
        let mut sjf = SJFscheduler::new();

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

        sjf.register(A);

        assert_eq!(Some(A), sjf.take());

        sjf.register(B);
        sjf.register(C);
        sjf.register(D);
        sjf.register(A);

        assert_eq!(Some(D), sjf.take());
        assert_eq!(Some(C), sjf.take());
        assert_eq!(Some(B), sjf.take());
        assert_eq!(Some(A), sjf.take());
        assert_eq!(None, sjf.take());


    }


}


use priority_queue::PriorityQueue;
use crate::{Task, SchedulingPolicy};

pub struct SJFscheduler {
    tasks: PriorityQueue<Task,i32>
}

impl SJFscheduler {
    pub fn new() -> Self{
        Self {
            tasks: PriorityQueue::new(),
        }
    }
}

impl SchedulingPolicy for SJFscheduler {

    fn register(&mut self, task: Task){
        self.tasks.push(task,0-(task.duration as i32));
    }

    fn take(&mut self)-> Option<Task>{
        if let Some((task,_)) = self.tasks.pop() {
            return Some(task);
        }
        return None;
    }
}


