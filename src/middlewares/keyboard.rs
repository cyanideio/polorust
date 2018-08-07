use redux::{DispatchFunc, Middleware, Store};
use store::action::AppAction;
use store::action::command::Phase;
use store::app::{AppState, AppMode, ModeCategory};
use termion::event::Key;

// use std::sync::mpsc;
// use store::events::Event;
// pub struct TxMiddleware {
//     pub tx: mpsc::Sender<Event>,
// }

pub struct KeyboardMiddleWare { }

impl Middleware<AppState> for KeyboardMiddleWare {
    fn dispatch(
        &self,
        store: &Store<AppState>,
        action: AppAction,
        next: &DispatchFunc<AppState>,
    ) -> Result<AppState, String> {
        debug!("[ACT]: {:?}", action);
        match &action {
            &AppAction::Keyboard(key) => {
                let _state = store.get_state();
                match get_key_action(key, _state) {
                    Ok(_action) => { let _ = store.dispatch(_action); }
                    Err(err) => { debug!("[ERR] {:?}", err) }
                }
            }
            _ => {}
        }
        return next(store, action);
    }
}

fn get_key_action(_key: Key, _state: AppState) -> Result<AppAction, String> {
    match _state.mode.category {
        ModeCategory::Normal => normal_key(_key, _state),
        ModeCategory::Command => command_key(_key, _state)
    }
}

fn normal_key (_key: Key, _state: AppState) -> Result<AppAction, String> {
    match _key {
        Key::Char(':') => Ok(AppAction::SetMode(AppMode::get_mode("command"))),
        _ => Err(String::from("Key not Implemented"))
    }
}   

fn command_key (_key: Key, mut state: AppState) -> Result<AppAction, String> {
    match _key {
        Key::Esc => Ok(AppAction::SetMode(AppMode::get_mode("normal"))),
        Key::Char('\n') => {
            let cmd = state.command.split_off(1);
            // let prompt_in = format_output!("green", "In", &cmd);
            // let _ = store.dispatch(AppAction::ConsoleWrite(prompt_in));
            Ok(AppAction::Command(Phase::Validate(cmd)))
        }
        Key::Char(_char) => Ok(AppAction::CommandBarPush(_char)),
        _  => Err(String::from("Key not Implemented")) 
    }
}
