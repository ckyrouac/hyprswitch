use crate::daemon::gui::gui_handle::{
    gui_close, gui_set_client
};
use crate::Share;
use gtk4::glib::clone;
use gtk4::prelude::GestureExt;
use gtk4::{EventSequenceState, GestureClick};
use hyprland::shared::Address;
use tracing::info;

pub(crate) fn click_client(share: &Share, address: &Address) -> GestureClick {
    let gesture = GestureClick::new();
    gesture.connect_pressed(clone!(
        #[strong]
        address,
        #[strong]
        share,
        move |gesture, _, _, _| {
            gesture.set_state(EventSequenceState::Claimed);
            info!("Switching to client {:?}", address.clone());
            gui_set_client(&share, address.clone());
            info!("Exiting on click of client window");
            gui_close(&share);
        }
    ));
    gesture
}
