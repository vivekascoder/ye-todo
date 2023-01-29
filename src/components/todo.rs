use crate::state::TodoState;
use crate::state::{SelectedTodo, ShowDialog, Todo};
use crate::utils::local_storage;
use gloo_console::log;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::{dispatch, prelude::*};

#[derive(Clone, Debug)]
struct CheckboxData {
    event: Event,
    todo: Todo,
}

#[function_component]
pub fn TodoComponent() -> Html {
    let (todo_state, dispatch) = use_store::<TodoState>();
    let use_effect_dispatch = dispatch.clone();
    use_effect_with_deps(
        move |_| {
            let dispatch = use_effect_dispatch.clone();
            let todos = local_storage::get_todos_from_localstorage();
            dispatch.clone().set(TodoState { todos });
        },
        (),
    );
    let value = use_state(|| String::default());
    let cloned_value = value.clone();
    let input_value = (*value).clone();

    let create_todo = dispatch.reduce_mut_callback(move |todo_state| {
        let now = js_sys::Date::get_milliseconds(&js_sys::Date::new_0()) as i32;
        let todo = Todo {
            id: now,
            title: value.clone().to_string(),
            completed: false,
        };
        todo_state.todos = todo_state
            .todos
            .iter()
            .cloned()
            .chain(Some(todo.clone()))
            .collect();
        local_storage::insert_todo_to_localstorage(todo.clone()).unwrap();
        value.set(String::from(""));
    });

    let on_value_change = Callback::from(move |event: Event| {
        cloned_value.set(event.target_unchecked_into::<HtmlInputElement>().value());
    });

    let handle_todo_delete_dispath = dispatch.clone();

    let handle_todo_delete = Callback::from(move |id: i32| {
        let dispatch = handle_todo_delete_dispath.clone();
        local_storage::delete_todo_from_localstorage(id).unwrap();
        dispatch
            .reduce_mut_callback(move |todo_state| {
                todo_state.todos = todo_state
                    .todos
                    .iter()
                    .cloned()
                    .filter(|t| t.id != id)
                    .collect();
            })
            .emit(());
    });

    let handle_checkbox_event = Callback::from(move |data: CheckboxData| {
        // let todos_state = todos_state.clone();
        let checked: bool = data
            .event
            .target_unchecked_into::<HtmlInputElement>()
            .checked();
        let todo = data.todo;
        local_storage::mark_todo_from_localstorage(todo.id, checked).unwrap();
        dispatch
            .reduce_mut_callback(move |todo_state| {
                todo_state.todos = todo_state
                    .todos
                    .iter()
                    .cloned()
                    .map(|t| {
                        if t.id == todo.id {
                            Todo {
                                id: t.id,
                                title: t.title,
                                completed: checked,
                            }
                        } else {
                            t
                        }
                    })
                    .collect();
            })
            .emit(());
    });

    let selected_todo = use_context::<UseStateHandle<SelectedTodo>>().unwrap();
    let show_dialog = use_context::<UseStateHandle<ShowDialog>>().unwrap();
    let set_selected_todo = Callback::from(move |todo: Todo| {
        selected_todo.set(SelectedTodo { todo: Some(todo) });
        show_dialog.set(ShowDialog {
            show: !show_dialog.show,
        });
    });

    html! {
        <div>
            <h1>
                {"Toddo App"}
            </h1>

            <h3>
                {"Active todos:"}
            </h3>
            <ul>
                {for todo_state.todos.iter().map(|todo| {
                    let on_todo_select = {
                        let handle_checkbox_event = handle_checkbox_event.clone();
                        let todo = todo.clone();
                        Callback::from(move |ev: Event| {
                            let checkbox_data = CheckboxData {
                                event: ev.clone(),
                                todo: todo.clone(),
                            };
                            handle_checkbox_event.emit(checkbox_data);
                        })
                    };

                    let on_todo_update  = {
                        let todo = todo.clone();
                        let set_selected_todo = set_selected_todo.clone();
                        Callback::from(move |_: MouseEvent| {
                            set_selected_todo.emit(todo.clone());
                        })
                    };

                    let on_todo_delete = {
                        let todo = todo.clone();
                        let handle_todo_delete = handle_todo_delete.clone();
                        Callback::from(move |_: MouseEvent| {
                            handle_todo_delete.emit(todo.id);
                        })
                    };

                    html! {
                        <li style={"margin-bottom: 0.3rem"}>
                            <button onclick={on_todo_delete}>{"🗑️"}</button>
                            <button onclick={on_todo_update}>{"✏️"}</button>
                            <input type="checkbox" checked={todo.completed} onchange={on_todo_select}/>
                            <span style={"margin-right: 1rem;"} class={
                                classes!(
                                    if todo.completed {
                                        Some("check")
                                    } else {
                                        None
                                    }
                                )
                            }>
                                {&todo.title}
                            </span>
                        </li>
                    }
                })}
            </ul>
            <div>
                <input type="text" name="content" placeholder={"Do something ..."}  value={input_value} onchange={on_value_change} />
                <button onclick={create_todo}>
                    {"➕"}
                </button>
            </div>
        </div>
    }
}
