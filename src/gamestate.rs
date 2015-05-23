// Gamestate model for SpaceColony
// I am not shure wether this should be a part of the controller or a model, but I think it is
// better if it is a model because the render engine has to be able to acess it.
//
// (c) 2015 Rahix

#[allow(dead_code)]
pub mod gamestate {
    // This can't be done via enums cause enums aren't able to be comparated
    // >:-( Yes, I needed a little while to get it work
    //                           _,..__,
    //                       ,.'''      `"-,_
    //                     ,'                '.
    //                   ,'                    '
    //                  /                       \_
    //                 ;     -.                   `\
    //                 |       |     _         _    .
    //                ;       ,'  ,-' `.     /' `.  |
    //                |       '  /  o   |   t  o  \.'    .,-.
    //                 |         |:    .'   |:    .|    /    \
    //                 ;         \:.._./    ':_..:/ `. |      L
    //                  \  ,-'           |\_         `\-     "'-.
    //      ,-"'``'-,    `f              '/`>                    `.
    //    ,'        `L___.|              '  `     . _,/            \
    //    |                \_          _   _    .-.]____,,r        |
    //    \             ,. ___""----./` \,' ',`\'  \      \     mx.'
    //      `'-'|        '`         `|   |   |  |  |       `'--"'`
    //          ,         |           L_.'.__:__.-'
    //           \        /
    //            `'-- "'`
    //
    // (http://chris.com/ascii/index.php?art=people/skeletons)
    pub mod states {
        pub const NO_STATE:i32 = 0;
        pub const INITIALIZING:i32 = 1;
        pub const MAIN_MENU:i32 = 2;
        pub const OPTIONS_MENU:i32 = 3;
        pub const IN_GAME:i32 = 4;
    }
    pub type State = i32;

    pub struct ChangeAction {
        new_state: State,
        last_state: State,
        simple: bool,
        action: fn()
    }

    impl ChangeAction {
        pub fn new(new_state: State, action: fn()) -> ChangeAction {
            ChangeAction {
                new_state: new_state,
                last_state: states::NO_STATE,
                simple: true,
                action: action
            }
        }

        pub fn add_last(&mut self, last_state: State) {
            self.last_state = last_state;
            self.simple = false;
        }

        pub fn new_with_last(new_state: State, last_state: State, action: fn()) -> ChangeAction {
            ChangeAction {
                new_state: new_state,
                last_state: last_state,
                simple: false,
                action: action
            }
        }

        pub fn run_action(&self) {
            let action = self.action;
            action();
        }
    }

    pub struct GameState {
        current_state: State,
        last_state: State,
        change_actions: Vec<ChangeAction>
    }

    impl GameState {
        pub fn new() -> GameState {
            GameState {
                current_state: states::NO_STATE,
                last_state: states::NO_STATE,
                change_actions: Vec::new()
            }
        }

        pub fn register_change_action(&mut self, action: ChangeAction) {
            self.change_actions.push(action);
        }

        fn trigger_change_actions(&mut self) {
            // Iterate through change actions and call all which match current state constitution
            for action in self.change_actions.iter() {
                if (action.new_state == self.current_state)
                   && (action.simple == true) {
                    action.run_action();
                }
                if (action.new_state == self.current_state)
                   && (action.last_state == self.last_state)
                   && (action.simple == false) {
                   action.run_action();
                }
            }
        }

        pub fn trigger_state_change(&mut self, new_state: State) {
            self.last_state = self.current_state;
            self.current_state = new_state;
            // Trigger change actions
            self.trigger_change_actions();
        }
    }
}
