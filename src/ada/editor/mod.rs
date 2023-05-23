use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

use crate::AdaParams;

#[derive(Lens)]
struct Data {
	params: Arc<AdaParams>,
}

impl Model for Data {}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
	ViziaState::new(|| (800, 600))
}

pub(crate) fn create(
	params: Arc<AdaParams>,
	editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
	create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
		assets::register_noto_sans_light(cx);
		assets::register_noto_sans_thin(cx);

		Data {
			params: params.clone(),
		}
		.build(cx);

		Label::new(cx, "you{r Mothre")
			.font_size(40f32)
			.left(Units::Percentage(50f32))
			.top(Units::Percentage(50f32));

		ResizeHandle::new(cx);

		// HStack::new(cx, |cx| {
		// 	Label::new(cx, "Ada")
		// 		.font_family(vec![FamilyOwned::Name(String::from(
		// 			assets::NOTO_SANS_BOLD,
		// 		))])
		// 		.font_size(15.0)
		// 		.height(Pixels(15.0))
		// 		.child_top(Pixels(15.0))
		// 		.child_left(Pixels(15.0));

		// 	Label::new(cx, "Gain");
		// 	ParamSlider::new(cx, Data::params, |params| &params.gain);
		// })
		// .col_between(Pixels(0.0))
		// .child_bottom(Stretch(1.0));
	})
}
