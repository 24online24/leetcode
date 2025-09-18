use std::collections::{BinaryHeap, HashMap};

struct TaskManager {
    tasks_info: HashMap<i32, (i32, i32)>,
    task_priorities: BinaryHeap<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut tasks_info = HashMap::new();
        let mut task_priorities = BinaryHeap::new();
        for user_task_priority in tasks {
            let (user_id, task_id, priority) = (
                user_task_priority[0],
                user_task_priority[1],
                user_task_priority[2],
            );
            tasks_info.insert(task_id, (user_id, priority));
            task_priorities.push((priority, task_id));
        }
        TaskManager {
            tasks_info,
            task_priorities,
        }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.tasks_info.insert(task_id, (user_id, priority));
        self.task_priorities.push((priority, task_id));
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        self.tasks_info
            .entry(task_id)
            .and_modify(|user_priority| user_priority.1 = new_priority);
        self.task_priorities.push((new_priority, task_id));
    }

    fn rmv(&mut self, task_id: i32) {
        self.tasks_info.remove(&task_id);
    }

    fn exec_top(&mut self) -> i32 {
        while let Some(priority_task) = self.task_priorities.pop() {
            let (priority, task_id) = (priority_task.0, priority_task.1);
            if let Some(user_priority) = self.tasks_info.get(&task_id) {
                if user_priority.1 != priority {
                    continue;
                }
                let user_id = user_priority.0;
                self.tasks_info.remove(&task_id);
                return user_id;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::TaskManager;

    #[test]
    fn example_1() {
        let mut task_manager =
            TaskManager::new(vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]]);
        task_manager.add(4, 104, 5);
        task_manager.edit(102, 8);
        assert_eq!(3, task_manager.exec_top());
        task_manager.rmv(101);
        task_manager.add(5, 105, 15);
        assert_eq!(5, task_manager.exec_top());
    }

    #[test]
    fn example_99() {
        let mut task_manager = TaskManager::new(vec![vec![10, 26, 25]]);
        task_manager.rmv(26);
        assert_eq!(5, task_manager.exec_top());
    }
}
