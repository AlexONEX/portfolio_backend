use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Transaction {
    id: u32,
    name: String,
    units: u32,
    adjusted_return: f64,
}

// Handler para obtener transacciones
async fn transactions() -> impl Responder {
    let sample_transactions = vec![
        Transaction {
            id: 1,
            name: "Empresa X".to_string(),
            units: 20,
            adjusted_return: 5.2,
        },
        Transaction {
            id: 2,
            name: "Empresa Y".to_string(),
            units: 15,
            adjusted_return: 3.8,
        },
    ];
    HttpResponse::Ok().json(sample_transactions)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/api/transactions", web::get().to(transactions)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
