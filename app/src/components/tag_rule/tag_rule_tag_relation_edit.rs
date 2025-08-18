use crate::components::property_selector::PropertySelector;
use crate::components::Component;
use egui::{Grid, Ui};
use idct_game::curiosity::tag::id::TagID;
use idct_game::curiosity::tag::rules::TagRuleTagRelation;

pub struct TagRuleTagRelationEdit<'a> {
    relation: &'a mut TagRuleTagRelation,
    cached_tag_ids: &'a [TagID],
    parent_tag_id: Option<&'a TagID>,
    id: &'a str,
}

impl<'a> TagRuleTagRelationEdit<'a> {
    pub fn new(relation: &'a mut TagRuleTagRelation, tag_ids: &'a [TagID]) -> Self {
        Self {
            relation,
            cached_tag_ids: tag_ids,
            parent_tag_id: None,
            id: "tag_rule_tag_relation_edit",
        }
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.id = id;
        self
    }

    pub fn parent_tag_id(mut self, parent_tag_id: &'a TagID) -> Self {
        self.parent_tag_id = Some(parent_tag_id);
        self
    }
}

impl Component for TagRuleTagRelationEdit<'_> {
    fn show(self, ui: &mut Ui) {
        Grid::new(self.id)
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Tag");
                PropertySelector::new(&mut self.relation.tag, self.cached_tag_ids, |tag_id| tag_id)
                    .id(self.id)
                    .display(|tag_id| tag_id.as_ref())
                    .condition(|tag_id| {
                        if tag_id.as_ref().is_empty() {
                            return false;
                        }

                        self.parent_tag_id
                            .map(|parent_tag_id| tag_id != parent_tag_id)
                            .unwrap_or(true)
                    })
                    .show(ui);
                ui.end_row();
                ui.end_row();

                ui.label("Factor");
                ui.add(
                    egui::Slider::new(&mut self.relation.factor, 0.0..=1.0)
                        .fixed_decimals(2)
                        .text(""),
                );
                ui.end_row();
            });
    }
}
