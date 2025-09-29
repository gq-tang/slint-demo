#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{process, sync::Arc};

use slint::{ComponentHandle, ModelRc, VecModel, quit_event_loop};
use user::{
    model::{self},
    repository,
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let dsn = repository::init_dsn("sqlite://data.db")?;
    let pool = repository::sqlite_connect(&dsn)
        .await
        .expect("sqlite connect error");

    repository::migrate_db(&pool)
        .await
        .expect("migrate db error");

    let arc_pool = Arc::new(pool);

    let app = model::ui::MainWindow::new()?;

    app.on_exit(move || {
        if let Err(e) = quit_event_loop() {
            println!("quit_event_loop error:{}", e);
            process::exit(1);
        }
    });

    let pool_clone = arc_pool.clone();
    let app_weak = app.as_weak();
    app.global::<model::ui::UserHandler>()
        .on_save_user(move |user| {
            let pool = pool_clone.clone();
            let app_weak = app_weak.clone();
            tokio::spawn(async move {
                let data: model::User = user.into();
                if let Err(e) = repository::save_user(pool.as_ref(), data).await {
                    _ = app_weak.upgrade_in_event_loop(move |app| {
                        app.invoke_show_dialog(slint::SharedString::from(format!(
                            "save failed:{}",
                            e
                        )));
                    });
                    return;
                } else {
                    _ = app_weak.upgrade_in_event_loop(move |app| {
                        app.invoke_show_dialog(slint::SharedString::from("save success"));
                    });
                }
            });
        });

    let pool_clone = arc_pool.clone();
    let app_weak = app.as_weak();

    app.global::<model::ui::UserHandler>()
        .on_list_user(move |search| {
            let pool = pool_clone.clone();
            let app_weak = app_weak.clone();
            tokio::spawn(async move {
                let users = match repository::list_user(&pool, &search).await {
                    Ok(users) => users,
                    Err(e) => {
                        _ = app_weak.upgrade_in_event_loop(move |app| {
                            app.invoke_show_dialog(slint::SharedString::from(format!(
                                "query failed:{}",
                                e
                            )));
                        });
                        return;
                    }
                };

                _ = app_weak.upgrade_in_event_loop(move |app| {
                    let row: VecModel<ModelRc<slint::StandardListViewItem>> = VecModel::default();
                    for user in users.into_iter() {
                        let row_data = VecModel::default();
                        let id = slint::StandardListViewItem::from(slint::format!("{}", user.id));
                        row_data.push(id);
                        let username =
                            slint::StandardListViewItem::from(slint::format!("{}", user.username));
                        row_data.push(username);
                        let age = slint::StandardListViewItem::from(slint::format!("{}", user.age));
                        row_data.push(age);
                        let gender =
                            slint::StandardListViewItem::from(slint::format!("{}", user.gender));
                        row_data.push(gender);
                        let email = slint::StandardListViewItem::from(slint::format!(
                            "{}",
                            user.email.unwrap_or_default()
                        ));
                        row_data.push(email);

                        row.push(ModelRc::new(row_data));
                    }

                    app.global::<model::ui::UserHandler>()
                        .set_users(ModelRc::new(row));
                });
            });
        });
    app.run()?;
    Ok(())
}
