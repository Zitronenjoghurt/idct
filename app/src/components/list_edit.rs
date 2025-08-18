use crate::components::Component;
use egui::{CollapsingHeader, ScrollArea, Ui};

pub struct ListEdit<'a, T, F, L>
where
    T: Default,
    F: Fn(&mut T, &mut Ui),
    L: Fn(&T) -> String,
{
    items: &'a mut Vec<T>,
    item_render: F,
    item_label: L,
    add_label: &'a str,
    max_height: Option<f32>,
    max_width: Option<f32>,
    id: &'a str,
}

impl<'a, T, F, L> ListEdit<'a, T, F, L>
where
    T: Default,
    F: Fn(&mut T, &mut Ui),
    L: Fn(&T) -> String,
{
    pub fn new(items: &'a mut Vec<T>, item_render: F, item_label: L) -> Self {
        Self {
            items,
            item_render,
            item_label,
            add_label: "Add",
            max_height: Some(400.0),
            max_width: Some(400.0),
            id: "list_edit",
        }
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = id;
        self
    }

    pub fn add_label(mut self, label: &'a str) -> Self {
        self.add_label = label;
        self
    }

    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }
}

impl<T, F, L> Component for ListEdit<'_, T, F, L>
where
    T: Default,
    F: Fn(&mut T, &mut Ui),
    L: Fn(&T) -> String,
{
    fn show(self, ui: &mut Ui) {
        ui.vertical(|ui| {
            let mut to_remove = None;

            let mut scroll_area = ScrollArea::vertical().id_salt(self.id);
            if let Some(max_height) = self.max_height {
                scroll_area = scroll_area.max_height(max_height);
            }
            if let Some(max_width) = self.max_width {
                scroll_area = scroll_area.max_width(max_width);
            }

            scroll_area.show(ui, |ui| {
                for (index, item) in self.items.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        if ui.button("🗑").clicked() {
                            to_remove = Some(index);
                        }
                        CollapsingHeader::new((self.item_label)(item))
                            .id_salt((self.id, index))
                            .show(ui, |ui| {
                                (self.item_render)(item, ui);
                            });
                    });
                }
            });

            if let Some(index) = to_remove {
                self.items.remove(index);
            }

            if ui.button(self.add_label).clicked() {
                self.items.push(T::default());
            }
        });
    }
}
