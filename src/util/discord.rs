use std::{thread, time};
use discord_presence::{Client, Event};

/// The discord application's client ID.
static CLIENT_ID: u64 = 1070156213892947978;

pub fn setup_presence() {
    let mut client = Client::new(CLIENT_ID);

    client.start();
    thread::sleep(time::Duration::from_secs(1));

    client.set_activity(|act| {
        act.assets(|a| a.large_image("logo"))
    })
    .unwrap();
}
