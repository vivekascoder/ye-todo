use std::{
    clone,
    time::{SystemTime, UNIX_EPOCH},
};
use web_sys::{
    console::{log, log_1},
    HtmlInputElement,
};
use yew::{html::IntoPropValue, prelude::*};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <TodoComponent />
        </div>
    }
}

#[derive(Clone, Debug)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Clone, Debug)]
struct CheckboxData {
    event: Event,
    todo: Todo,
}

#[function_component]
fn TodoComponent() -> Html {
    let todos = vec![
        Todo {
            id: 1,
            title: "Todo 1".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Todo 2".to_string(),
            completed: false,
        },
        Todo {
            id: 3,
            title: "Todo 3".to_string(),
            completed: false,
        },
    ];

    let todos_state = use_state(|| todos);
    let dc_todos_state = todos_state.clone();
    let cloned_todos_state = todos_state.clone();
    let render_todos_state = todos_state.clone();
    let value = use_state(|| String::from("Do XYZ"));
    let cloned_value = value.clone();
    let input_value = (*value).clone();

    let create_todo = Callback::from(move |_| {
        let todo = Todo {
            id: 100,
            title: value.clone().to_string(),
            completed: false,
        };
        cloned_todos_state.set(
            cloned_todos_state
                .clone()
                .iter()
                .cloned()
                .chain(Some(todo))
                .collect(),
        );
    });

    let on_value_change = Callback::from(move |event: Event| {
        cloned_value.set(event.target_unchecked_into::<HtmlInputElement>().value());
    });

    let handle_todo_delete = Callback::from(move |id: i32| {
        let todos_state = dc_todos_state.clone();
        todos_state.set(
            todos_state
                .clone()
                .iter()
                .cloned()
                .filter(|t| t.id != id)
                .collect(),
        );
    });

    let handle_checkbox_event = Callback::from(move |data: CheckboxData| {
        let todos_state = todos_state.clone();
        let checked: bool = data
            .event
            .target_unchecked_into::<HtmlInputElement>()
            .checked();
        let todo = data.todo;
        todos_state.set(
            todos_state
                .clone()
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
                .collect(),
        );
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
                {for render_todos_state.iter().map(|todo| {
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

                    let on_todo_delete = {
                        let todo = todo.clone();
                        let handle_todo_delete = handle_todo_delete.clone();
                        Callback::from(move |_: MouseEvent| {
                            handle_todo_delete.emit(todo.id);
                        })
                    };

                    html! {
                        <li style={"margin-bottom: 0.3rem"}>
                            <input type="checkbox" onchange={on_todo_select}/>
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
                            <button onclick={on_todo_delete}>{"delete"}</button>
                        </li>
                    }
                })}
            </ul>
            <div>
                <input type="text" name="content"  value={input_value} onchange={on_value_change} />
                <button onclick={create_todo}>
                    {"Create Todo"}
                </button>
            </div>
        </div>
    }
}
