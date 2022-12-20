pub mod editor;
use editor::*;
pub mod terminal;
use terminal::*;

fn main() {
    let mut editor = Editor::default();

    editor.run()
}
