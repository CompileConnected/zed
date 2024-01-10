use gpui::{img, prelude::*, AnyElement, SharedUrl};
use smallvec::SmallVec;
use ui::prelude::*;

#[derive(IntoElement)]
pub struct CollabNotification {
    avatar_uri: SharedUrl,
    accept_button: Button,
    dismiss_button: Button,
    children: SmallVec<[AnyElement; 2]>,
}

impl CollabNotification {
    pub fn new(
        avatar_uri: impl Into<SharedUrl>,
        accept_button: Button,
        dismiss_button: Button,
    ) -> Self {
        Self {
            avatar_uri: avatar_uri.into(),
            accept_button,
            dismiss_button,
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for CollabNotification {
    fn children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.children
    }
}

impl RenderOnce for CollabNotification {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_stack()
            .text_ui()
            .justify_between()
            .size_full()
            .overflow_hidden()
            .elevation_3(cx)
            .p_2()
            .gap_2()
            .child(img(self.avatar_uri).w_12().h_12().rounded_full())
            .child(v_stack().overflow_hidden().children(self.children))
            .child(
                v_stack()
                    .child(self.accept_button)
                    .child(self.dismiss_button),
            )
    }
}
