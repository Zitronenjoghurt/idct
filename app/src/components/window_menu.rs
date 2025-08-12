use crate::windows::ViewWindow;
use egui::{Ui, WidgetText};

pub struct WindowMenu<'a> {
    windows: Vec<&'a mut dyn WindowMenuWindow>,
    title: &'a str,
}

impl<'a> WindowMenu<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            windows: Vec::new(),
            title,
        }
    }

    pub fn window<W: ViewWindow>(mut self, window: &'a mut W) -> Self {
        self.windows.push(window);
        self
    }

    pub fn show(self, ui: &mut Ui) {
        ui.heading(self.title);
        ui.separator();

        for window in self.windows {
            let mut open = window.window_is_open();
            if ui.checkbox(&mut open, window.window_title()).changed() {
                window.window_set_open(open);
            }
        }
    }
}

trait WindowMenuWindow {
    fn window_is_open(&self) -> bool;
    fn window_set_open(&mut self, open: bool);
    fn window_title(&self) -> WidgetText;
}

impl<T> WindowMenuWindow for T
where
    T: ViewWindow,
{
    fn window_is_open(&self) -> bool {
        self.is_open()
    }

    fn window_set_open(&mut self, open: bool) {
        self.set_open(open)
    }

    fn window_title(&self) -> WidgetText {
        self.title().into()
    }
}
