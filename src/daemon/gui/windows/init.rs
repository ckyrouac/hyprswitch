use crate::daemon::gui::icon::set_icon;
use crate::daemon::gui::windows::click::click_client;
use crate::daemon::gui::MonitorData;
use crate::{ClientData, Share, WorkspaceData};
use gtk4::{prelude::*, Fixed, Frame, Image, Overflow, Overlay};
use hyprland::shared::{Address, WorkspaceId};

fn scale(value: i16, size_factor: f64) -> i32 {
    (value as f64 / 30.0 * size_factor) as i32
}

pub fn init_windows(
    share: Share,
    workspaces_p: &[(WorkspaceId, WorkspaceData)],
    clients_p: &[(Address, ClientData)],
    monitor_data: &mut MonitorData,
    size_factor: f64,
) {
    clear_monitor(monitor_data);

    let (width, height) = if let Some((_, workspace_data)) = workspaces_p.first() {
        (workspace_data.width, workspace_data.height) // Return a tuple here
    } else {
        (20, 20) // Provide a fallback tuple if no workspace exists
    };

    let clients = clients_p
        .iter()
        .collect::<Vec<_>>();

    for (address, client) in &clients {
        let mut label = client.title.clone();
        let dup = clients.clone().iter().any(|(_, client_data)| client_data.title == client.title && client_data.pid != client.pid);
        if dup {
            label = client.class.clone();
        }

        if label.len() > 20 {
            label = format!("{}...", &label[0..17]);
        }

        let client_fixed = Fixed::builder()
            .width_request(scale(width.try_into().unwrap(), size_factor))
            .height_request(scale(height.try_into().unwrap(), size_factor))
            .build();

        let client_frame = Frame::builder()
            .width_request(30)
            .label(label)
            .label_xalign(0.5)
            .child(&client_fixed)
            .build();

        let image = Image::builder()
            .css_classes(vec!["client-image"])
            .pixel_size(
                // (scale(client.height, size_factor).clamp(50, 200) as f64 / 1.5) as i32 - 20,
                130 //TODO: dynamic size
            )
            .build();
        set_icon(&client.class, client.pid, &image);
        client_frame.set_child(Some(&image));

        let client_overlay = {
            let client_overlay = Overlay::builder()
                .css_classes(vec!["client", "background"])
                .overflow(Overflow::Hidden)
                .child(&client_frame)
                .build();
            client_overlay.add_controller(click_client(&share, address));
            monitor_data.workspaces_flow.insert(&client_overlay, -1);
            client_overlay
        };
        monitor_data
            .client_refs
            .insert(address.clone(), (client_overlay, None));
    }
}

fn clear_monitor(monitor_data: &mut MonitorData) {
    // remove all children
    while let Some(child) = monitor_data.workspaces_flow.first_child() {
        monitor_data.workspaces_flow.remove(&child);
    }
    // remove previous overlay from monitor
    if let Some(overlay_ref_label) = monitor_data.workspaces_flow_overlay.1.take() {
        monitor_data
            .workspaces_flow_overlay
            .0
            .remove_overlay(&overlay_ref_label);
    }

    // remove active class from monitor
    monitor_data
        .workspaces_flow_overlay
        .0
        .remove_css_class("monitor_active");
}
