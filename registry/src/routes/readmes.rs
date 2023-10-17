use crate::{errors::Result, readme::GithubApiClient, repository::Registry};
use actix_web::{get, post, web, HttpResponse};

#[post("/extensions/details/{extension_name}/readme")]
pub async fn fetch_and_save_readme(
    path: web::Path<String>,
    registry: web::Data<Registry>,
    client: web::Data<GithubApiClient>,
) -> Result<HttpResponse> {
    let extension_name = path.into_inner();
    crate::readme::fetch_and_save_readme(client.as_ref(), registry.as_ref(), &extension_name)
        .await?;

    Ok(HttpResponse::Ok().finish())
}

#[get("/extensions/details/{extension_name}/readme")]
pub async fn get_readme(
    path: web::Path<String>,
    registry: web::Data<Registry>,
) -> Result<HttpResponse> {
    let extension_name = path.into_inner();
    let readme = registry.get_extension_readme(&extension_name).await?;

    Ok(HttpResponse::Ok().body(readme))
}
