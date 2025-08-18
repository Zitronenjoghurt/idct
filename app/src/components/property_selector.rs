use crate::components::Component;
use egui::Ui;

pub struct PropertySelector<'a, T, I, P, D, C>
where
    P: Fn(&I) -> &T,
    D: Fn(&T) -> &str,
    C: Fn(&I) -> bool,
    T: Clone + PartialEq,
{
    id: &'a str,
    value: &'a mut T,
    items: &'a [I],
    get_property: P,
    display: D,
    condition: C,
}

impl<'a, T, I, P> PropertySelector<'a, T, I, P, fn(&T) -> &str, fn(&I) -> bool>
where
    P: Fn(&I) -> &T,
    T: Clone + PartialEq,
{
    pub fn new(value: &'a mut T, items: &'a [I], get_property: P) -> Self {
        Self {
            value,
            items,
            get_property,
            display: |_| "",
            condition: |_| true,
            id: "property_selector",
        }
    }
}

impl<'a, T, I, P, D, C> PropertySelector<'a, T, I, P, D, C>
where
    P: Fn(&I) -> &T,
    D: Fn(&T) -> &str,
    C: Fn(&I) -> bool,
    T: Clone + PartialEq,
{
    pub fn display<NewD>(self, display: NewD) -> PropertySelector<'a, T, I, P, NewD, C>
    where
        NewD: Fn(&T) -> &str,
    {
        PropertySelector {
            value: self.value,
            items: self.items,
            get_property: self.get_property,
            display,
            condition: self.condition,
            id: self.id,
        }
    }

    pub fn condition<NewC>(self, condition: NewC) -> PropertySelector<'a, T, I, P, D, NewC>
    where
        NewC: Fn(&I) -> bool,
    {
        PropertySelector {
            value: self.value,
            items: self.items,
            get_property: self.get_property,
            display: self.display,
            condition,
            id: self.id,
        }
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = id;
        self
    }
}

impl<T, I, P, D, C> Component for PropertySelector<'_, T, I, P, D, C>
where
    P: Fn(&I) -> &T,
    D: Fn(&T) -> &str,
    C: Fn(&I) -> bool,
    T: Clone + PartialEq,
{
    fn show(self, ui: &mut Ui) {
        let selected_text = (self.display)(self.value);

        egui::ComboBox::from_id_salt(self.id)
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                for item in self.items {
                    if !(self.condition)(item) {
                        continue;
                    }

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
