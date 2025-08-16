use crate::components::Component;
use egui::{ScrollArea, Ui};

pub struct ListEdit<'a, T, F>
where
    F: Fn(&mut T, &mut Ui),
{
    items: &'a mut Vec<T>,
    item_render: F,
    add_label: &'a str,
    max_height: Option<f32>,
}

impl<'a, T, F> ListEdit<'a, T, F>
where
    F: Fn(&mut T, &mut Ui),
{
    pub fn new(items: &'a mut Vec<T>, item_render: F) -> Self {
        Self {
            items,
            item_render,
            add_label: "Add",
            max_height: Some(400.0),
        }
    }

    pub fn add_label(mut self, label: &'a str) -> Self {
        self.add_label = label;
        self
    }

    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }
}

impl<T, F> Component for ListEdit<'_, T, F>
where
    T: Default,
    F: Fn(&mut T, &mut Ui),
{
    fn show(self, ui: &mut Ui) {
        let mut to_remove = None;

        let mut scroll_area = ScrollArea::vertical();
        if let Some(max_height) = self.max_height {
            scroll_area = scroll_area.max_height(max_height);
        }

        scroll_area.show(ui, |ui| {
            for (index, item) in self.items.iter_mut().enumerate() {
                ui.push_id(index, |ui| {
                    ui.horizontal(|ui| {
                        (self.item_render)(item, ui);
                        if ui.button("🗑").clicked() {
                            to_remove = Some(index);
                        }
                    });
                    ui.separator();
                });
            }
        });

        if let Some(index) = to_remove {
            self.items.remove(index);
        }

        if ui.button(self.add_label).clicked() {
            self.items.push(T::default());
        }
    }
}
