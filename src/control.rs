use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::event::{Axis, Button, GamepadId, KeyCode, KeyMods};
use ggez::graphics;

use crate::menu::Menu;
use crate::game::{Game, GameOptions};

#[repr(u8)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub enum ProgramState {
    Menu,
    Game,
}

pub struct Control {
    state: ProgramState,
    menu: Option<Menu>,
    game: Option<Game>,
    game_options: Option<GameOptions>,
}

impl Control {
    pub fn new(ctx: &mut Context) -> Control {
        Self {
            state: ProgramState::Menu,
            menu: Some(Menu::new(ctx)),
            game: None,
            game_options: None,
        }
    }

    pub fn change_state(&mut self, ctx: &mut Context, new_state: ProgramState) {
        self.state = match new_state {
            ProgramState::Menu => {
                self.menu = Some(Menu::new(ctx));
                ProgramState::Menu
            },
            ProgramState::Game => {
                // TODO: is there a way to not manually copy this? I implemented Copy and Clone to the input::ControlScheme struct, but it doesn't work on a vector of them
                let mut vec_keyboard_inputs: Vec<crate::inputs::ControlScheme> = Vec::with_capacity(self.game_options.as_ref().expect("[!] attempted to start Game with no GameOptions").vec_keyboard_inputs.len());
                for control_scheme in self.game_options.as_ref().expect("[!] attempted to start Game with no GameOptions").vec_keyboard_inputs.iter() {
                    vec_keyboard_inputs.push(*control_scheme);
                }
                self.game = Some(Game::new(ctx,
                    self.game_options.as_ref().expect("[!] attempted to start Game with no GameOptions").num_players,
                    self.game_options.as_ref().expect("[!] attempted to start Game with no GameOptions").starting_level,
                    vec_keyboard_inputs,
                ));
                ProgramState::Game
            },
        };
    }
}

// this is run once every frame and passes control off to whichever state the game is in
impl EventHandler for Control {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            ProgramState::Menu => {
                // update the menu and get the state with GameOptions if ProgramState is changing
                if let Some(state_and_gameoptions) = self.menu.as_mut()
                    .expect("[!] control.state == ProgramState::Menu but control.menu == None")
                    .update()
                {
                    self.menu = None;
                    self.game_options = Some(state_and_gameoptions.1);
                    self.change_state(ctx, state_and_gameoptions.0);
                }
            },
            ProgramState::Game => {
                // update the game and get the state that the program should be in
                let state_returned = self.game.as_mut()
                    .expect("[!] control.state == ProgramState::Game but control.game == None")
                    .update();
                // should we change states?
                if self.state != state_returned {
                    self.game = None;
                    self.change_state(ctx, state_returned);
                }
            },
        };

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        repeat: bool,
    ) {
        match self.state {
            ProgramState::Menu => self.menu.as_mut()
                .expect("[!] control.state == ProgramState::Menu but control.menu == None")
                .key_down_event(keycode, repeat),
            ProgramState::Game => self.game.as_mut()
                .expect("[!] control.state == ProgramState::Game but control.game == None")
                .key_down_event(keycode, repeat),
        };
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods
    ) {
        match self.state {
            ProgramState::Menu => self.menu.as_mut()
                .expect("[!] control.state == ProgramState::Menu but control.menu == None")
                .key_up_event(keycode),
            ProgramState::Game => self.game.as_mut()
                .expect("[!] control.state == ProgramState::Game but control.game == None")
                .key_up_event(keycode),
        };
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            ProgramState::Menu => self.menu.as_mut()
                .expect("[!] control.state == ProgramState::Menu but control.menu == None")
                .draw(ctx),
            ProgramState::Game => self.game.as_mut()
                .expect("[!] control.state == ProgramState::Game but control.game == None")
                .draw(ctx),
        };

        graphics::present(ctx)
    }

    // this seems unused but is called somewhere in ggez to ultimately make things scale and get placed correctly when changing window size
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let new_rect = graphics::Rect::new(0.0, 0.0, width, height);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}