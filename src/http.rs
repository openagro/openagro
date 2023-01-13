/// Copyright (c) 2023 OpenAgro Developers
///
/// This file under Lesser General Public License v3.0, please read accompanying file
/// copy or, read on https://www.gnu.org/licenses/lgpl-3.0.html

use actix_web::{ HttpServer as HS, App, web, HttpResponse};


pub async fn run(port: u16){
    HS::new( || {
        print!("Should Run");
        App::new()
        .route("/", web::get().to(|| HttpResponse::Ok()))
    }).bind(("localhost", port))
    .expect("Error")
    .run()
    .await
    .unwrap()
}