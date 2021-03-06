use crate::*;

pub const NO_STATE_CHANGE: usize = 0;

pub trait State {
    fn enter(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread);
    fn run(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) -> usize;
    fn leave(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread);
}

pub struct StateGroup {
    pub states: Vec<Box<dyn State>>,
    pub current: usize,
}

pub fn create_state_group() -> StateGroup {
    StateGroup {
        states: vec![],
        current: 0,
    }
}

pub fn add_state(
    group: &mut StateGroup,
    thread: &mut RaylibThread,
    rl: &mut RaylibHandle,
    new_state: Box<dyn State>,
) {
    group.states.push(new_state);
    if group.current == 0 {
        group.states[group.current].enter(rl, thread);
    }
}

pub fn set_state(
    group: &mut StateGroup,
    thread: &mut RaylibThread,
    rl: &mut RaylibHandle,
    new_state: usize,
) {
    group.states[group.current].leave(rl, thread);
    group.current = new_state;
    group.states[group.current].enter(rl, thread);
}

pub fn run_current_state(group: &mut StateGroup, thread: &mut RaylibThread, rl: &mut RaylibHandle) {
    let ret = group.states[group.current].run(rl, thread);
    if ret != NO_STATE_CHANGE {
        set_state(group, thread, rl, ret - 1);
    }
}
