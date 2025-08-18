#[derive(Debug)]
pub enum ContentEditorAction {
    RenameCuriosityProperty { old: String, new: String },
}
