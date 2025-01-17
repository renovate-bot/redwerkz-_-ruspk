use crate::models::*;
use crate::utils;
use crate::AppData;
use actix_web::{delete, error, get, web, Error, HttpRequest, HttpResponse};
use actix_web_grants::proc_macro::has_any_role;
use anyhow::Result;

/// retrieve all versions
#[get("/api/version")]
pub async fn get_all(req: HttpRequest, data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let (limit, offset, q) = utils::handle_query_parameters(req.query_string());
    let conn = data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbVersion::find_all(&conn, limit, offset, q))
        .await
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?;
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/version")]
#[has_any_role("ADMIN", "PACKAGE_ADMIN")]
pub async fn delete(post_data: web::Json<utils::IdType>, app_data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let conn = app_data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbVersion::delete(&conn, post_data.id))
        .await
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?;
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/version/{id}")]
#[has_any_role("ADMIN", "PACKAGE_ADMIN")]
pub async fn delete_id(path: web::Path<i32>, app_data: web::Data<AppData>) -> Result<HttpResponse, Error> {
    let id = path.into_inner();
    let conn = app_data.pool.get().expect("couldn't get db connection from pool");
    let response = web::block(move || DbVersion::delete(&conn, id))
        .await
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?
        .map_err(|e| {
            debug!("{}", e);
            error::ErrorInternalServerError(e)
        })?;
    Ok(HttpResponse::Ok().json(response))
}
