use gloo_console::log;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::{dispatch, prelude::use_store};

use crate::{
    state::{SelectedTodo, ShowDialog, Todo, TodoState},
    utils::local_storage,
};

#[function_component]
pub fn EditDialog() -> Html {
    let show_dialog = use_context::<UseStateHandle<ShowDialog>>().unwrap();
    let cloned_show_dialog = show_dialog.clone();
    let handle_toggle = Callback::from(move |_: MouseEvent| {
        let show_dialog = cloned_show_dialog.clone();
        log!(show_dialog.show);
        show_dialog.set(ShowDialog {
            show: !show_dialog.show,
        });
    });

    let updated_todo_title = use_state(|| "".to_string());
    let selected_todo = use_context::<UseStateHandle<SelectedTodo>>().unwrap();
    let cloned_selected_todo = (*selected_todo).clone();
    let cloned_updated_todo_title = (*updated_todo_title).clone();
    let a = updated_todo_title.clone();
    let b = updated_todo_title.clone();

    use_effect_with_deps(
        move |_| {
            log!("Changed!");
            let value = match cloned_selected_todo.todo {
                Some(todo) => todo.title,
                None => "".to_string(),
            };

            b.clone().set(value);
        },
        (selected_todo.clone()),
    );

    let (_, dispatch) = use_store::<TodoState>();

    // God, please save me from this mess.
    let update_todo_title_cloned_selected_todo = (*selected_todo).clone();
    let update_todo_title_cloned_updated_todo_title = (*updated_todo_title).clone();
    let handle_toggle_clone = handle_toggle.clone();

    let update_todo_title = dispatch.reduce_mut_callback(move |todo_state| {
        let selected_todo = update_todo_title_cloned_selected_todo.clone();
        let updated_todo_title = update_todo_title_cloned_updated_todo_title.clone();
        let todo = selected_todo.todo.unwrap();

        todo_state.todos = todo_state
            .todos
            .iter()
            .map(|t| {
                if t.id == todo.id {
                    Todo {
                        id: t.id,
                        title: updated_todo_title.clone(),
                        completed: t.completed,
                    }
                } else {
                    t.clone()
                }
            })
            .collect();

        // Hide the dialog
        handle_toggle_clone
            .clone()
            .emit(MouseEvent::new("click").unwrap());

        // Update in local storage
        local_storage::update_todo_title(todo.id, updated_todo_title.clone()).unwrap();
    });

    html! {
        <div class={classes!(
            if show_dialog.show { Some("hidden")} else { Some("show")},
        )} style="position: fixed; top: 2rem; left: 50%; transform: translateX(-50%); background: rgba(0,0,0,1); width: 20rem; border: 1px solid white;">
            <div style="position: relative; margin: 0.8rem;">
                <button style="position: absolute; right: 0; top: 0" onclick={handle_toggle}>{"❌"}</button>
                <h3>{"Edit Dialog"}</h3>
                <p> {"This is a dialog to edit a todo item."} </p>

                <div style="display: flex;">
                    <input onchange={Callback::from(move |event: Event| {
                        a.clone().set(event.target_unchecked_into::<HtmlInputElement>().value());
                    })} value={cloned_updated_todo_title.clone()} type="text" placeholder="Title" style="flex-grow: 1;" />
                    <button onclick={update_todo_title}>{"✅"}</button>
                </div>
            </div>
        </div>
    }
}
