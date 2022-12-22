pub mod document;
pub mod editor;
pub mod position;
pub mod row;
pub mod terminal;

use editor::*;
use position::*;

fn main() {
    let mut editor = Editor::default();

    editor.run()
}
