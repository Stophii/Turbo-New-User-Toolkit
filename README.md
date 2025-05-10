# New User Toolkit

## Description

A few easy to implement functions to help you start off in Turbo!

>💡 **Tip** 💡 This [video](https://youtu.be/g6EFmxjdR_o) goes over this repo

## Text Wrapping

Writing text is widely used in making games—whether it's for item descriptions, player abilities, or simple menus. Sometimes that text can get long, and since Turbo's resolution is pretty small by default, you might find your text spilling off the screen. It's only natural that we address that issue with `wrap_textbetter`, a function to help out with that problem.

Copy and paste this into your project wherever you like!

>💡 **Tip** 💡 You can add everything in this tutorial below the `Turbo::go!` loop, but making separate files is never a bad idea!

```rust
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
```

Once you have `wrap_textbetter` in your project you can call it like this

```rust
    let text = wrap_textbetter("Write whatever you want here", 10);
    text!(&text, x = 5, y = 100, color = 0xffffffff);
```

All you need to do is bind it to a reference—in this case, I used `text`—and then write your text inside quotes `""`. The final step is to specify a character limit; your sentence will start a new line when this limit is reached. If you happen to hit the character limit in the middle of a word, it’ll make sure to wrap early! It also takes care of empty spaces by starting each new line with text.

This is an example using the function to display text with a character limit of 29 and the `large` font.


```rust
    let text = wrap_textbetter("Wrapping text makes my life easier, hello world!", 29);
    text!(&text, x = 15, y = 100, font = "large", color = 0xffffffff); 
```
If you're writing text a lot for your project, make sure to make your life easier with `wrap_textbetter`!

>💡 **Tip** 💡 Make sure to specify an `x`, `y`, `font` and `color` for your text, as well as the correct `&reference` so it displays properly.

## State Machine

When initializing a project, you'll have a single `Turbo::go!` loop to work within. Having a **state machine** will simplify your project, and it's really easy to set up—you just need one function and an enum! The `state_of_game` function will hold your different screen code loops. The `enum Screen` will allow you to specify what those screens are.


```rust
fn state_of_game(state: &mut GameState) {
    match state.screen {
        Screen::Title => {
            clear(0xffffffff);
            //clearing screen white!
            if gamepad(0).start.just_pressed() {
                state.screen = Screen::Game;
            }

        },
        Screen::Game => {
            clear(0x99a3a3ff);
            //clearing screen grey!
            if gamepad(0).start.just_pressed() {
                state.screen = Screen::Gameover;
            }
        },
        Screen::Gameover => {
            clear(0x000000ff);
            //clearing screen black!
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
```

Once you have these two building blocks located in your project you can replace your `Turbo::go!` with this

```rust

turbo::init! {
    struct GameState {
        screen: Screen,
    } = {
        Self {
            screen: Screen::Title,
        }
    }
}

turbo::go! {
    let mut state = GameState::load();

    state_of_game(&mut state); 

    state.save();
}
```

And everything will run nice and smooth!

>💡 **Tip** 💡 You can add in as many different variants as you need and just make sure you add them to the `enum` and then make sure you have a way to get to them whether it's a `gamepad` input or a specific requirement!

## Timers 

The last thing I have for you is a basic `Timer` struct to help you get started with implementing time-based events more easily! You can create simpler timers than this, but this version gives you a bit more functionality. Whether you're thinking of adding a short delay after an input or displaying a timer on screen in real time, this will help you do that with ease!

```rust
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
```

To use this timer we'll need to intialize it in the gamestate just like the **State Machine** above. Go ahead and add this to your `Turbo::init!`

```rust
turbo::init! {
    struct GameState {
        timer: Timer,
    } = {
        GameState {
            timer: Timer::new(25.0, 60.0),
        }
    }
}
```
this will intialize a timer upon game start with a duration of 25 seconds. The `60.0` is the frames per second and turbo runs at ~60.0 frames per second!

Adding a new timer is as simple as this

```rust
    if gamepad(0).a.just_pressed() {
        state.timer = Timer::new(60.0, 60.0)
    }
```

and if you want to display the time left as a countdown?

```rust
    let time_left = state.timer.get_time_left(60.0);
    let timer_text = format!("{:.2} sec", time_left);
    text!(&timer_text, x = 20.0, y = 20.0, font = "medium", color = 0xff0000ff);
```

or maybe a stop watch?

```rust
    let stop_watch = state.timer.get_stopwatch_time(60.0);
    let stopwatch_text = format!("{:.2} sec", stop_watch);
    text!(&stopwatch_text, x = 20.0, y = 40.0, font = "medium", color = 0xff0000ff);
```

Timers are incredibly useful, feel free to use `is_done` and `reset` for whatever scenarios you'd need them for! `is_done` can be used for if you want something to happen when the timer finishes

```rust
    if state.timer.is_done() {
        state.screen = Screen::Gameover
    }
```
>💡 **Tip** 💡 adding a `_` infront of an innactive function will get rid of any warnings in your terminal! i.e. `_get_time_left`

If any of these functions ended up helping you make sure to let me know in the official [Turbo discord](https://discord.gg/V5YWWvQvKW)!







