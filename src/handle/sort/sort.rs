use std::collections::VecDeque;

use hyprland::shared::Address;

use crate::ClientData;

/// Sorts clients with complex sorting
///
/// * 'clients' - Vector of clients to sort
/// * 'ignore_workspaces' - Don't split clients into workspaces (treat all clients on monitor as one workspace)
/// * 'ignore_monitors' - Don't split clients into monitors (treat all clients as one monitor)
pub fn sort_clients(
    clients: Vec<(Address, ClientData)>,
) -> Vec<(Address, ClientData)> {
    let mut sorted_clients = clients.clone();

    sorted_clients.sort_by(|(_, a), (_, b)| {
        a.focus_history_id.cmp(&b.focus_history_id)
    });
    let mut queue: VecDeque<(Address, ClientData)> = VecDeque::from(clients);

    let mut line_start = queue.pop_front();
    while let Some((current_addr, current)) = line_start {
        sorted_clients.push((current_addr, current));
        line_start = queue.pop_front();
    }

    sorted_clients
}
