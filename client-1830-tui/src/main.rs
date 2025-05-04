use bevy_quinnet::client::certificate::CertificateVerificationMode;
use bevy_quinnet::client::connection::ClientEndpointConfiguration;
use bevy_quinnet::shared::channels::ChannelsConfiguration;
use bevy_quinnet::client::QuinnetClient;
use std::net::Ipv6Addr;
use bevy::app::AppExit;
use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy_ratatui::event::KeyEvent;
use bevy_ratatui::{RatatuiPlugins, terminal::RatatuiContext};
use crossterm::event::KeyCode;

fn main() {
    let frame_time = std::time::Duration::from_secs_f32(1. / 60.);

    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(frame_time)),
            RatatuiPlugins::default(),
        ))
        .add_systems(Update, (input_system, draw_system))
        .run();
}

fn draw_system(mut context: ResMut<RatatuiContext>) -> Result {
    context.draw(|frame| {
        let text = ratatui::text::Text::raw("hello world");
        frame.render_widget(text, frame.area());
    })?;

    Ok(())
}

fn input_system(mut events: EventReader<KeyEvent>, mut exit: EventWriter<AppExit>) {
    for event in events.read() {
        if let KeyCode::Char('q') = event.code {
            exit.send_default();
        }
    }
}

fn start_connection(mut client: ResMut<QuinnetClient>) {
    client
        .open_connection(
            ClientEndpointConfiguration::from_ips(
                Ipv6Addr::LOCALHOST,
                6000,
                Ipv6Addr::UNSPECIFIED,
                0,
            ),
            CertificateVerificationMode::SkipVerification,
            ChannelsConfiguration::default(),
        );

    // When trully connected, you will receive a ConnectionEvent
}

fn handle_server_messages(
    mut client: ResMut<QuinnetClient>,
    /*...*/
) {
    while let Ok(Some((channel_id, message))) = client.connection_mut().receive_message::<ServerMessage>() {
        match message {
            // Match on your own message types ...
            ServerMessage::ClientConnected { client_id, username} => {/*...*/}
            ServerMessage::ClientDisconnected { client_id } => {/*...*/}
            ServerMessage::ChatMessage { client_id, message } => {/*...*/}
        }
    }
}
