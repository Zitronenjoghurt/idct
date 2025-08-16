use crate::components::Component;
use egui::Ui;

pub struct PropertySelector<'a, T, I, P, D>
where
    P: Fn(&I) -> &T,
    D: Fn(&T) -> &str,
    T: Clone + PartialEq,
{
    id: &'a str,
    value: &'a mut T,
    items: &'a [I],
    get_property: P,
    display: D,
}

impl<'a, T, I, P, D> PropertySelector<'a, T, I, P, D>
where
    P: Fn(&I) -> &T,
    D: Fn(&T) -> &str,
    T: Clone + PartialEq,
{
    pub fn new(value: &'a mut T, items: &'a [I], get_property: P, display: D) -> Self {
        Self {
            value,
            items,
            get_property,
            display,
            id: "property_selector",
        }
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = id;
        self
    }
}

impl<T, I, P, D> Component for PropertySelector<'_, T, I, P, D>
where
    P: Fn(&I) -> &T,
    D: Fn(&T) -> &str,
    T: Clone + PartialEq,
{
    fn show(self, ui: &mut Ui) {
        let selected_text = (self.display)(self.value);

        egui::ComboBox::from_id_salt(self.id)
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                for item in self.items {
                    let property = (self.get_property)(item);
                    let is_selected = self.value == property;

                    if ui
                        .selectable_label(is_selected, (self.display)(property))
                        .clicked()
                    {
                        *self.value = property.clone();
                    }
                }
            });
    }
}
