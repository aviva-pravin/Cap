// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

fn main() {
    // We have to hold onto the ClientInitGuard until the very end
    let _guard = match std::env::var("CAP_DESKTOP_SENTRY_URL") {
        Ok(sentry_url) => Some(sentry::init((
            sentry_url,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                debug: cfg!(debug_assertions),
                before_send: Some(Arc::new(|event| {
                    #[cfg(debug_assertions)]
                    {
                        let msg = event.message.clone().unwrap_or("No message".into());
                        println!("Sentry captured {}: {}", &event.level, &msg);
                        println!("Sentry user: {:?}", &event.user);
                        Some(event)
                    }

                    #[cfg(not(debug_assertions))]
                    {
                        Some(event)
                    }
                })),
                ..Default::default()
            },
        ))),
        Err(_) => {
            tracing::warn!(
                "Sentry URL not found in environment variables, skipping Sentry initialization"
            );
            None
        }
    };

    #[cfg(debug_assertions)]
    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            username: Some("_DEV_".into()),
            ..Default::default()
        }));
    });

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build multi threaded tokio runtime")
        .block_on(desktop_solid_lib::run());
}
