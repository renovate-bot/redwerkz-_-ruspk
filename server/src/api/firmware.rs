use crate::models::*;
use actix_web::{Error, HttpRequest, HttpResponse, get, web};
use crate::{AppData, DbConn};
use anyhow::Result;
use crate::utils;

fn db_get_firmware(conn: &DbConn, limit: i64, offset: i64) -> Result<Vec<DbFirmware>> {
    Ok(DbFirmware::find_all(conn, limit, offset)?)
}

/// retrieve all firmware
#[get("/firmware")]
pub async fn get_firmware(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error>{
    let (limit, offset) = utils::paginate_qs(req.query_string());
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || db_get_firmware(&conn, limit, offset)).await.map_err(|e| {
        debug!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(response))
}