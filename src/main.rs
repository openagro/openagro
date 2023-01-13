use extension::ExtensionMeta;

/// Copyright (c) 2023 OpenAgro Developers
///
/// This file under Lesser General Public License v3.0, please read accompanying file
/// copy or, read on https://www.gnu.org/licenses/lgpl-3.0.html

mod extension;
mod http;

/// main entry of openagro
fn main() {
    let xt = extension::ExtensionMetaData{
        name: Some("".to_string()),
        module_id: Some("".to_string()),
        author: Some("".to_string()),
        data: vec![],
        depends: vec![],
    };

    xt.auto_migration();
    env_logger::init();
    actix_web::rt::System::with_tokio_rt(||{
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(8)
            .thread_name("openagro_thread")
            .build()
            .unwrap()
    }).block_on(http::run(8000));
}