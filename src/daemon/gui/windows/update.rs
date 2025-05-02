use crate::{Active, SharedData};
use crate::daemon::gui::MonitorData;

macro_rules! update_type {
    (
        $htypr_data:expr, $identifier_name:ident, $css_active_name:expr, $id:expr,
        $overlay:expr, $label:expr, $active:expr, $gui_config:expr, $submap_info:expr, $valign: expr
    ) => {
        use gtk4::prelude::WidgetExt;
        let find = $htypr_data.iter().find(|(i, _)| *i == $id);
        if let Some((_, data)) = find {
            if data.enabled {
                // mark the active client
                if !$gui_config.hide_active_window_border && $active == $id {
                    $overlay.add_css_class($css_active_name);
                } else {
                    $overlay.remove_css_class($css_active_name);
                }
            } else {
                // remove label if exists
                if let Some(label) = $label.take() {
                    $overlay.remove_overlay(&label);
                }
                $overlay.remove_css_class($css_active_name);
            }
        }
    };
}

pub fn update_windows(gui_monitor_data: &mut MonitorData, data: &SharedData) -> anyhow::Result<()> {
    match &data.active {
        Some(Active::Client(addr)) => {
            for (id, (overlay, label)) in gui_monitor_data.client_refs.iter_mut() {
                update_type!(
                    data.hypr_data.clients,
                    address,
                    "client_active",
                    *id,
                    overlay,
                    label,
                    *addr,
                    &data.gui_config,
                    &data.submap_config,
                    Align::End
                );
            }
        }
        Some(Active::Workspace(active_id)) => {
            for (wid, (overlay, label)) in gui_monitor_data.workspace_refs.iter_mut() {
                update_type!(
                    data.hypr_data.workspaces,
                    id,
                    "workspace_active",
                    *wid,
                    overlay,
                    label,
                    *active_id,
                    &data.gui_config,
                    &data.submap_config,
                    Align::Start
                );
            }
        }
        Some(Active::Monitor(active_id)) => {
            let (overlay, label) = &mut gui_monitor_data.workspaces_flow_overlay;
            update_type!(
                data.hypr_data.monitors,
                id,
                "monitor_active",
                gui_monitor_data.id,
                overlay,
                label,
                *active_id,
                &data.gui_config,
                &data.submap_config,
                Align::Start
            );
        }
        _ => {}
    }
    Ok(())
}
