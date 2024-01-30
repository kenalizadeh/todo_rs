use std::fmt::Display;

#[derive(Debug)]
pub struct Todo {
    id: u8,
    content: String,
    done: String,
    created_at: String,
    done_at: String
}

impl Todo {
    pub fn new(
        id: u8,
        content: String,
        done: bool,
        created_at: String,
        done_at: String
    ) -> Self {
        Todo {
            id,
            content,
            done: if done { "yes".to_string() } else { "no".to_string() },
            created_at,
            done_at,
        }
    }
}

pub fn map_display(todo: &Todo) -> Vec<&dyn Display> {
    vec![&todo.id, &todo.content, &todo.done, &todo.created_at, &todo.done_at]
}