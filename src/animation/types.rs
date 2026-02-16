use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UserAction {
    Dance, // 跳舞 - 行1 (4-7)
    Sleep, // 睡觉 - 行2 (8-11)
    Talk,  // 聊天 - 行3 (12-15)
    Feed,  // 喂食 - 行4 (16-19)
    Pet,   // 抚摸 - 行5 (20-23)
}

#[derive(Resource)]
pub struct ActionStateMachine {
    pub state: ActionState,
    pub(crate) timer: Timer,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ActionState {
    #[default]
    Idle,
    Acting(UserAction),
}

impl Default for ActionStateMachine {
    fn default() -> Self {
        Self {
            state: ActionState::Idle,
            timer: Timer::from_seconds(2.5, TimerMode::Once),
        }
    }
}

impl ActionStateMachine {
    pub fn trigger(&mut self, action: UserAction) {
        self.state = ActionState::Acting(action);
        self.timer.reset();
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.state, ActionState::Idle)
    }

    pub(crate) fn current_action(&self) -> Option<UserAction> {
        match self.state {
            ActionState::Idle => None,
            ActionState::Acting(action) => Some(action),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
