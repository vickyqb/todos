use ic_cdk::{export_candid,query,update,storage};
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
        storage::stable_save((todos.clone(),)).expect("Could not save todos");
        todo_id 
    });
    todo_id 
}

#[query]
fn get_todos() -> Vec<Todo> {
    TODOS.with(|todos| {
        // todos.borrow().clone()
        storage::stable_restore::<(Vec<Todo>,)>().unwrap_or_else(|_| (Vec::new(),)).0
    })
}

#[update]
fn mark_done(todo_id: u64) {
    TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        if let Some(todo) = todos.iter_mut().find(|todo| todo.todo_id == todo_id) {
            todo.done = true;
        }
        storage::stable_save((todos.clone(),)).expect("Could not save todos");
    });
}

#[update]
fn remove_todo_by_id(todo_id: u64)-> String {
    let todos = storage::stable_restore::<(Vec<Todo>,)>().unwrap_or_else(|_| (Vec::new(),));
    TODOS.with(|todos| {
        let mut todos = todos.borrow_mut();
        todos.retain(|todo| todo.todo_id != todo_id);
        storage::stable_save((todos.clone(),)).expect("Could not save todos");
    });
    return "Todo removed".to_string();
}


export_candid!();