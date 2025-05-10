turbo::init! {
    struct GameState {
        screen: Screen,
        timer: Timer,
    } = {
        GameState {
            screen: Screen::Title,
            timer: Timer::new(0.0, 60.0),
        }
    }
}

turbo::go! {
    let mut state = GameState::load();

    state_of_game(&mut state); 

    if gamepad(0).a.just_pressed() {
        state.timer.reset()
    }

    if state.screen != Screen::Title {
        if state.timer.is_done() {
            state.screen = Screen::Gameover
        } else {
            state.screen = Screen::Game
        }
    }

    state.save();
}

fn state_of_game(state: &mut GameState) {
    match state.screen {
        Screen::Title => {
            clear(0xffffffff);
            //clearing screen white!
            if gamepad(0).start.just_pressed() {
                state.screen = Screen::Game;
                state.timer = Timer::new(5.0, 60.0);
            }

            let text = wrap_textbetter("Tutorial press `space` to play", 7);
            text!(&text, x = 110, y = 50, color = 0x000000ff);

        },
        Screen::Game => {
            clear(0x99a3a3ff);
            let time_left = state.timer.get_time_left(60.0);
            let timer_text = format!("{:.2} sec", time_left);
            text!(&timer_text, x = 110.0, y = 40.0, font = "medium", color = 0xff0000ff);

            let stop_watch = state.timer.get_stopwatch_time(60.0);
            let stopwatch_text = format!("{:.2} sec", stop_watch);
            text!(&stopwatch_text, x = 110.0, y = 80.0, font = "medium", color = 0xff0000ff);

            //clearing screen grey!
            if gamepad(0).start.just_pressed() {
                state.screen = Screen::Gameover;
            }
        },
        Screen::Gameover => {
            clear(0x000000ff);
            //clearing screen black!
            let text = wrap_textbetter("Gameover! Press `z` to try again press `space` to go back to the title!", 48);
            text!(&text, x = 15, y = 55, color = 0xff0000ff);

            if gamepad(0).start.just_pressed() {
                state.screen = Screen::Title;
            }
        },
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub enum Screen {
    Title,
    Game,
    Gameover,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, PartialEq)]
pub struct Timer {
    pub start_tick: usize,
    pub duration_ticks: usize,
}

impl Timer {
    pub fn new(duration_seconds: f32, fps: f32) -> Self {
        Self {
            start_tick: tick(),
            duration_ticks: (duration_seconds * fps) as usize,
        }
    }

    pub fn is_done(&self) -> bool {
        tick() >= self.start_tick + self.duration_ticks
    }

    pub fn get_time_left(&self, fps: f32) -> f32 {
        let frames_left = self.duration_ticks.saturating_sub(tick() - self.start_tick);
        frames_left as f32 / fps
    }

    pub fn get_stopwatch_time(&self, fps: f32) -> f32 {
        let frames_elapsed = tick().saturating_sub(self.start_tick);
        let capped_frames = frames_elapsed.min(self.duration_ticks);
        capped_frames as f32 / fps
    }
    

    pub fn reset(&mut self) {
        self.start_tick = tick();
    }
}

pub fn wrap_textbetter(text: &str, max_line_length: usize) -> String {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if !current_line.is_empty() && current_line.len() + word.len() + 1 > max_line_length {
            lines.push(current_line);
            current_line = String::new();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines.join("\n")
}