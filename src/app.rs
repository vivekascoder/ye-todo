use todo_app::components::edit_dialog::EditDialog;
use todo_app::components::todo::TodoComponent;
use todo_app::state::{SelectedTodo, ShowDialog};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let show_dialog = use_state(|| ShowDialog { show: true });
    let selected_todo = use_state(|| SelectedTodo { todo: None });
    html! {
        <ContextProvider<UseStateHandle<ShowDialog>> context={show_dialog}>
            <ContextProvider<UseStateHandle<SelectedTodo>> context={selected_todo}>
            <div>
                <TodoComponent />
                <EditDialog />
            </div>
            </ContextProvider<UseStateHandle<SelectedTodo>>>
        </ContextProvider<UseStateHandle<ShowDialog>>>
    }
}
