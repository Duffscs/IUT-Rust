
#[cfg(test)]
mod tests {
    use super::EDFscheduler;
    use super::super::{SchedulingPolicy,Task};
    #[test]
    fn test_sjf() {
        let mut sjf = EDFscheduler::new();

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

        sjf.register(C);
        sjf.register(B);
        sjf.register(A);
        sjf.register(D);

        assert_eq!(Some(C), sjf.take());
        assert_eq!(Some(B), sjf.take());
        assert_eq!(Some(A), sjf.take());
        assert_eq!(Some(D), sjf.take());
        assert_eq!(None, sjf.take());


    }


}


use priority_queue::PriorityQueue;
use crate::{Task, SchedulingPolicy};

pub struct EDFscheduler {
    tasks: PriorityQueue<Task,i32>
}

impl EDFscheduler {
    pub fn new() -> Self{
        Self {
            tasks: PriorityQueue::new(),
        }
    }
}

impl SchedulingPolicy for EDFscheduler {

    fn register(&mut self, task: Task){
        self.tasks.push(task,0-(task.deadline as i32));
    }

    fn take(&mut self)-> Option<Task>{
        if let Some((task,_)) = self.tasks.pop() {
            return Some(task);
        }
        return None;
    }
}


