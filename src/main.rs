use yew::prelude::*;
mod gui;
mod game;
use gui::*;
use game::*;

fn main() {
    yew::Renderer::<BoardGUI>::new().render();
}