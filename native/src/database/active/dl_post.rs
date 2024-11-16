use std::ops::Deref;

use anyhow::Context;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::{EntityTrait, IntoActiveModel};
use sea_orm::{QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};

use crate::database::active::ACTIVE_DATABASE;
use crate::database::{create_index, create_index_a, create_table_if_not_exists, index_exists};
use crate::entities::Post;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "dl_post")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub host: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub tags: String,
    pub created_at: i64,
    pub creator_id: Option<i64>,
    pub author: String,
    pub change: i64,
    pub source: String,
    pub score: i64,
    pub md5: String,
    pub file_size: i64,
    pub file_url: String,
    pub is_shown_in_index: bool,
    pub preview_url: String,
    pub preview_width: i64,
    pub preview_height: i64,
    pub actual_preview_width: i64,
    pub actual_preview_height: i64,
    pub sample_url: String,
    pub sample_width: i64,
    pub sample_height: i64,
    pub sample_file_size: i64,
    pub jpeg_url: String,
    pub jpeg_width: i64,
    pub jpeg_height: i64,
    pub jpeg_file_size: i64,
    pub rating: String,
    pub has_children: bool,
    pub parent_id: Option<i64>,
    pub width: i64,
    pub height: i64,
    pub status: String,
    pub is_held: bool,
    pub frames_pending_string: String,
    // pub frames_pending: Vec<Value>,
    pub frames_string: String,
    // pub frames: Vec<Value>,
    //////////////////////////////
    pub dl_key: String,
    pub dl_created_time: i64, // timestamp milliseconds
    pub dl_status: i32,       // 0:未下载, 1:下载成功 2:下载失败 3:删除中
    pub file_format: String,
}

pub(crate) async fn init() {
    let gdb = ACTIVE_DATABASE.get().unwrap().lock().await;
    let db = gdb.deref();
    create_table_if_not_exists(db, Entity).await;
    if !index_exists(db, Entity {}.table_name(), "idx_dl_post_dl_created_time").await {
        create_index(
            db,
            Entity {}.table_name(),
            vec!["dl_created_time"],
            "idx_dl_post_dl_created_time",
        )
        .await;
    }
    if !index_exists(db, Entity {}.table_name(), "uk_dl_post_dl_key").await {
        create_index_a(
            db,
            Entity {}.table_name(),
            vec!["dl_key"],
            "uk_dl_post_dl_key",
            true,
        )
        .await;
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn append_dl_post(host: String, post: Post) -> anyhow::Result<bool> {
    let db = ACTIVE_DATABASE
        .get()
        .with_context(|| "error code (10025)")?
        .lock()
        .await;
    if let Some(_) = Entity::find()
        .filter(Column::Host.eq(host.clone()))
        .filter(Column::Id.eq(post.id.clone()))
        .one(db.deref())
        .await?
    {
        return Ok(false);
    }
    let dl_key = hex::encode(md5::compute(format!("{}|{}", host, post.id)).to_vec());
    Model {
        host,
        id: post.id,
        tags: post.tags,
        created_at: post.created_at,
        creator_id: post.creator_id,
        author: post.author,
        change: post.change,
        source: post.source,
        score: post.score,
        md5: post.md5,
        file_size: post.file_size,
        file_url: post.file_url,
        is_shown_in_index: post.is_shown_in_index,
        preview_url: post.preview_url,
        preview_width: post.preview_width,
        preview_height: post.preview_height,
        actual_preview_width: post.actual_preview_width,
        actual_preview_height: post.actual_preview_height,
        sample_url: post.sample_url,
        sample_width: post.sample_width,
        sample_height: post.sample_height,
        sample_file_size: post.sample_file_size,
        jpeg_url: post.jpeg_url,
        jpeg_width: post.jpeg_width,
        jpeg_height: post.jpeg_height,
        jpeg_file_size: post.jpeg_file_size,
        rating: post.rating,
        has_children: post.has_children,
        parent_id: post.parent_id,
        width: post.width,
        height: post.height,
        status: post.status,
        is_held: post.is_held,
        frames_pending_string: post.frames_pending_string,
        frames_string: post.frames_string,
        dl_key,
        dl_created_time: chrono::Local::now().timestamp_millis(),
        dl_status: 0,
        file_format: "".to_string(),
    }
    .into_active_model()
    .insert(db.deref())
    .await?;
    Ok(true)
}

pub(crate) async fn first_need_delete_post() -> anyhow::Result<Option<Model>> {
    let db = ACTIVE_DATABASE
        .get()
        .with_context(|| "error code (10025)")?
        .lock()
        .await;
    Ok(Entity::find()
        .filter(Column::DlStatus.eq(3))
        .one(db.deref())
        .await?)
}

pub(crate) async fn delete_download_info(dl_key: String) -> anyhow::Result<()> {
    let db = ACTIVE_DATABASE
        .get()
        .with_context(|| "error code (10025)")?
        .lock()
        .await;
    Entity::delete_many()
        .filter(Column::DlKey.eq(dl_key))
        .exec(db.deref())
        .await?;
    Ok(())
}

pub(crate) async fn first_not_result_post() -> anyhow::Result<Option<Model>> {
    let db = ACTIVE_DATABASE
        .get()
        .with_context(|| "error code (10025)")?
        .lock()
        .await;
    Ok(Entity::find()
        .filter(Column::DlStatus.eq(0))
        .one(db.deref())
        .await?)
}

#[allow(dead_code)]
pub(crate) async fn update_status_and_format(
    dl_key: String,
    status: i32,
    format: String,
) -> anyhow::Result<()> {
    let db = ACTIVE_DATABASE
        .get()
        .with_context(|| "error code (10025)")?
        .lock()
        .await;
    Entity::update_many()
        .filter(Column::DlKey.eq(dl_key))
        .col_expr(Column::DlStatus, Expr::value(status))
        .col_expr(Column::FileFormat, Expr::value(format))
        .exec(db.deref())
        .await?;
    Ok(())
}

pub(crate) async fn update_status(dl_key: String, status: i32) -> anyhow::Result<()> {
    let db = ACTIVE_DATABASE
        .get()
        .with_context(|| "error code (10025)")?
        .lock()
        .await;
    Entity::update_many()
        .filter(Column::DlKey.eq(dl_key))
        .col_expr(Column::DlStatus, Expr::value(status))
        .exec(db.deref())
        .await?;
    Ok(())
}

pub(crate) async fn all_downloads() -> anyhow::Result<Vec<Model>> {
    let db = ACTIVE_DATABASE
        .get()
        .with_context(|| "error code (10025)")?
        .lock()
        .await;
    Ok(Entity::find()
        .order_by_desc(Column::DlCreatedTime)
        .all(db.deref())
        .await?)
}

pub async fn reset_failed() -> anyhow::Result<()> {
    let db = ACTIVE_DATABASE
        .get()
        .with_context(|| "error code (10025)")?
        .lock()
        .await;
    Entity::update_many()
        .filter(Column::DlStatus.eq(2))
        .col_expr(Column::DlStatus, Expr::value(0))
        .exec(db.deref())
        .await?;
    Ok(())
}
