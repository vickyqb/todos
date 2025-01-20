use ic_cdk::{export_candid,query,update};
use std::cell::RefCell;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone)]
struct Todo {
    todo_id: u64,
    subject: String,
    description: String,
    done: bool,
}

thread_local! {
    static TODOS: RefCell<Vec<Todo>> = RefCell::new(Vec::new());
}


#[update]
fn create_todos(subject: String, description: String) -> u64{
    let todo_id = TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        let todo_id  = todos.len() as u64;
        todos.push(Todo {
            todo_id,
            subject,
            description,
            done: false,
        });
        todo_id 
    });
    todo_id 
}

#[query]
fn get_todos() -> Vec<Todo> {
    TODOS.with(|todos| {
        todos.borrow().clone()
    })
}

#[update]
fn mark_done(todo_id: u64) {
    TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        if let Some(todo) = todos.iter_mut().find(|todo| todo.todo_id == todo_id) {
            todo.done = true;
        }
    });
}

#[update]
fn remove_todo_by_id(todo_id: u64) {
    TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        todos.retain(|todo| todo.todo_id != todo_id);
    });
}


export_candid!();