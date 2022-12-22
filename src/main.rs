pub mod editor;
use editor::*;
pub mod terminal;
use terminal::*;
pub mod position;
use position::*;

fn main() {
    let mut editor = Editor::default();

    editor.run()
}
