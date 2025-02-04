use actix_web::rt::time::Instant;
use actix_web::{post, web, Responder};
use collection::operations::consistency_params::ReadConsistency;
use collection::operations::types::{RecommendRequest, RecommendRequestBatch};
use segment::types::ScoredPoint;
use storage::content_manager::errors::StorageError;
use storage::content_manager::toc::TableOfContent;

use super::read_params::ReadParams;
use crate::actix::helpers::process_response;

async fn do_recommend_points(
    toc: &TableOfContent,
    collection_name: &str,
    request: RecommendRequest,
    read_consistency: Option<ReadConsistency>,
) -> Result<Vec<ScoredPoint>, StorageError> {
    toc.recommend(collection_name, request, read_consistency)
        .await
}

#[post("/collections/{name}/points/recommend")]
pub async fn recommend_points(
    toc: web::Data<TableOfContent>,
    path: web::Path<String>,
    request: web::Json<RecommendRequest>,
    params: web::Query<ReadParams>,
) -> impl Responder {
    let name = path.into_inner();
    let timing = Instant::now();

    let response = do_recommend_points(
        toc.get_ref(),
        &name,
        request.into_inner(),
        params.consistency,
    )
    .await;

    process_response(response, timing)
}

async fn do_recommend_batch_points(
    toc: &TableOfContent,
    collection_name: &str,
    request: RecommendRequestBatch,
    read_consistency: Option<ReadConsistency>,
) -> Result<Vec<Vec<ScoredPoint>>, StorageError> {
    toc.recommend_batch(collection_name, request, read_consistency)
        .await
}

#[post("/collections/{name}/points/recommend/batch")]
pub async fn recommend_batch_points(
    toc: web::Data<TableOfContent>,
    path: web::Path<String>,
    request: web::Json<RecommendRequestBatch>,
    params: web::Query<ReadParams>,
) -> impl Responder {
    let name = path.into_inner();
    let timing = Instant::now();

    let response = do_recommend_batch_points(
        toc.get_ref(),
        &name,
        request.into_inner(),
        params.consistency,
    )
    .await;

    process_response(response, timing)
}

// Configure services
pub fn config_recommend_api(cfg: &mut web::ServiceConfig) {
    cfg.service(recommend_points)
        .service(recommend_batch_points);
}
