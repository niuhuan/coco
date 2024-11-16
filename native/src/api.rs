use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use reqwest::{Client, ClientBuilder};
use tokio::sync::{Mutex, RwLock};

use crate::database::active::dl_post;
use crate::database::cache::{image_cache, web_cache};
use crate::database::properties::property;
use crate::entities::{Post, PostPage, Tag, TagData, TagNetworkResponse};
use crate::local::join_paths;
use crate::utils::{from_str, hash_lock};
use crate::{block_on, get_downloads_dir, get_image_cache_dir};

pub fn init(root: String, downloads_to: String) {
    crate::init_root(&root, &downloads_to);
}

lazy_static! {
    static ref CLIENT: RwLock<Arc<Client>> = RwLock::new(Arc::new(
        ClientBuilder::new()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.88 Safari/537.36")
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap()
    ));
}

async fn client() -> Arc<Client> {
    CLIENT.read().await.clone()
}

pub(crate) async fn down_image(url: String) -> Result<(bytes::Bytes, String)> {
    let bytes = client()
        .await
        .get(url.clone())
        .header("referer", {
            if url.contains("yande.re") {
                "https://yande.re/"
            } else if url.contains("konachan.com") {
                "https://konachan.com/"
            } else if url.contains("konachan.net") {
                "https://konachan.net/"
            } else {
                ""
            }
        })
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    let format = image::guess_format(&bytes)?;
    let format = if let Some(format) = format.extensions_str().first() {
        format.to_string()
    } else {
        "".to_string()
    };
    Ok((bytes, format))
}

pub fn set_proxy(url: String) -> Result<()> {
    block_on(async {
        let mut builder = ClientBuilder::new()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.88 Safari/537.36")
            .timeout(Duration::from_secs(30));
        if url.as_str() != "" {
            builder = builder.proxy(reqwest::Proxy::all(url)?);
        }
        (*CLIENT.write().await) = Arc::new(builder.build()?);
        Ok(())
    })
}

pub fn desktop_root() -> Result<String> {
    #[cfg(target_os = "windows")]
    {
        Ok(join_paths(vec![
            std::env::current_exe()?
                .parent()
                .with_context(|| "error")?
                .to_str()
                .with_context(|| "error")?,
            "data",
        ]))
    }
    #[cfg(target_os = "macos")]
    {
        let home = std::env::var_os("HOME")
            .with_context(|| "error")?
            .to_str()
            .with_context(|| "error")?
            .to_string();
        Ok(join_paths(vec![
            home.as_str(),
            "Library",
            "Application Support",
            "niuhuan",
            "coco",
        ]))
    }
    #[cfg(target_os = "linux")]
    {
        let home = std::env::var_os("HOME")
            .with_context(|| "error")?
            .to_str()
            .with_context(|| "error")?
            .to_string();
        Ok(join_paths(vec![home.as_str(), ".niuhuan", "coco"]))
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    panic!("未支持的平台")
}

pub fn http_get(url: String) -> Result<String> {
    block_on(http_get_inner(url))
}

async fn http_get_inner(url: String) -> Result<String> {
    Ok(reqwest::ClientBuilder::new()
        .user_agent("coco")
        .build()?
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?)
}

pub fn save_property(k: String, v: String) -> Result<()> {
    block_on(property::save_property(k, v))
}

pub fn load_property(k: String) -> Result<String> {
    block_on(property::load_property(k))
}

lazy_static! {
    static ref TAG_SUMMARY_LOCK: Mutex::<()> = Mutex::new(());
}

pub fn tag_summary(host: String) -> anyhow::Result<TagData> {
    block_on(async {
        let _ = TAG_SUMMARY_LOCK.lock().await;
        web_cache::cache_first(
            format!("{}$TAG_SUMMARY", host),
            Duration::from_secs(60 * 60 * 24 * 365 * 99),
            Box::pin(async move {
                let tag_network_response: TagNetworkResponse = from_str(
                    &client()
                        .await
                        .get(&format!("{}/tag/summary.json", host))
                        .send()
                        .await?
                        .error_for_status()?
                        .text()
                        .await?,
                )?;
                let reg = regex::Regex::new("(\\d+)(`(\\S+`)+) ")?;
                let mut tags = Vec::<Tag>::new();
                for c in reg.captures_iter(&tag_network_response.data) {
                    let count: i64 = c.get(1).with_context(|| "err")?.as_str().parse()?;
                    let names: Vec<String> = c
                        .get(2)
                        .with_context(|| "err")?
                        .as_str()
                        .trim_matches('`')
                        .split("`")
                        .map(|s| s.to_owned())
                        .collect_vec();
                    tags.push(Tag {
                        image_total: count,
                        tag_names: names,
                    })
                }
                Ok(TagData {
                    version: tag_network_response.version,
                    tags,
                })
            }),
        )
        .await
    })
}

pub fn load_posts(host: String, tags: String, page: i64) -> anyhow::Result<PostPage> {
    block_on(web_cache::cache_first(
        format!("{}$LOAD_POSTS${}${}", host, tags, page),
        Duration::from_secs(60 * 60 * 2),
        Box::pin(async move {
            let data = client()
                .await
                .get(&if host.contains("rule34") {
                    format!("{}/index.php?page=post&tags={}&page={}", host, tags, page)
                } else {
                    format!("{}/post?&tags={}&page={}", host, tags, page)
                })
                .send()
                .await?
                .error_for_status()?
                .text()
                .await?;
            let reg = regex::Regex::new("Post\\.register\\((\\{[^\n]+\\})\\)\n")?;
            let mut posts = Vec::<Post>::new();
            for c in reg.captures_iter(&data) {
                let json = c.get(1).with_context(|| "err")?;
                let post: Post = from_str(json.as_str())?;
                posts.push(post);
            }
            let mut pages: i64 = 0;
            let reg = regex::Regex::new(">(\\d+)</a> <a class=\"next_page\"")?;
            for c in reg.captures_iter(&data) {
                pages = c.get(1).with_context(|| "err")?.as_str().parse()?;
            }
            Ok(PostPage {
                posts,
                page_total: pages,
            })
        }),
    ))
}

pub fn load_cache_image(
    url: String,
    useful: String,
    extends_field_int_first: Option<i32>,
    extends_field_int_second: Option<i32>,
    extends_field_int_third: Option<i32>,
) -> Result<LocalImage> {
    block_on(async {
        let _ = hash_lock(&url).await;
        if let Some(model) = image_cache::load_image_by_url(url.clone()).await? {
            image_cache::update_cache_time(url).await?;
            Ok(LocalImage {
                abs_path: join_paths(vec![
                    get_image_cache_dir().as_str(),
                    model.local_path.as_str(),
                ]),
                local_path: model.local_path,
                image_format: model.image_format,
                image_width: model.image_width,
                image_height: model.image_height,
            })
        } else {
            let local_path = hex::encode(md5::compute(&url).as_slice());
            let abs_path = join_paths(vec![get_image_cache_dir().as_str(), &local_path]);
            let (bytes, format) = down_image(url.clone()).await?;
            let image = image::load_from_memory(&bytes)?;
            tokio::fs::write(&abs_path, &bytes).await?;
            let model = image_cache::Model {
                url,
                useful,
                extends_field_int_first,
                extends_field_int_second,
                extends_field_int_third,
                local_path,
                cache_time: chrono::Local::now().timestamp_millis(),
                image_format: format,
                image_width: image.width(),
                image_height: image.height(),
            };
            let model = image_cache::insert(model).await?;
            Ok(LocalImage {
                abs_path,
                local_path: model.local_path,
                image_format: model.image_format,
                image_width: model.image_width,
                image_height: model.image_height,
            })
        }
    })
}

pub struct LocalImage {
    pub abs_path: String,
    pub local_path: String,
    pub image_format: String,
    pub image_width: u32,
    pub image_height: u32,
}

pub fn auto_clean(time: i64) -> Result<()> {
    let dir = get_image_cache_dir();
    block_on(async {
        loop {
            let caches: Vec<image_cache::Model> = image_cache::take_100_cache(time.clone()).await?;
            if caches.is_empty() {
                break;
            }
            for cache in caches {
                let local = join_paths(vec![dir.as_str(), cache.local_path.as_str()]);
                image_cache::delete_by_url(cache.url).await?; // 不管有几条被作用
                let _ = std::fs::remove_file(local); // 不管成功与否
            }
        }
        web_cache::clean_web_cache_by_time(time).await?;
        crate::database::cache::vacuum().await?;
        Ok(())
    })
}

pub fn clean_all_cache() -> Result<()> {
    auto_clean(1000 * 60 * 60 * 24 * 365 * 1024)
}

pub fn add_download_post(host: String, post: Post) -> Result<bool> {
    block_on(dl_post::append_dl_post(host, post))
}

pub struct DlPost {
    pub host: String,
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
    pub dl_status: i32,       // 0:未下载, 1:下载成功 2:下载失败
    pub file_format: String,
}

pub fn all_downloads() -> Result<Vec<DlPost>> {
    let downloads: Vec<dl_post::Model> = block_on(dl_post::all_downloads())?;
    Ok(downloads
        .iter()
        .map(|e| {
            let e = e.clone();
            DlPost {
                host: e.host,
                id: e.id,
                tags: e.tags,
                created_at: e.created_at,
                creator_id: e.creator_id,
                author: e.author,
                change: e.change,
                source: e.source,
                score: e.score,
                md5: e.md5,
                file_size: e.file_size,
                file_url: e.file_url,
                is_shown_in_index: e.is_shown_in_index,
                preview_url: e.preview_url,
                preview_width: e.preview_width,
                preview_height: e.preview_height,
                actual_preview_width: e.actual_preview_width,
                actual_preview_height: e.actual_preview_height,
                sample_url: e.sample_url,
                sample_width: e.sample_width,
                sample_height: e.sample_height,
                sample_file_size: e.sample_file_size,
                jpeg_url: e.jpeg_url,
                jpeg_width: e.jpeg_width,
                jpeg_height: e.jpeg_height,
                jpeg_file_size: e.jpeg_file_size,
                rating: e.rating,
                has_children: e.has_children,
                parent_id: e.parent_id,
                width: e.width,
                height: e.height,
                status: e.status,
                is_held: e.is_held,
                frames_pending_string: e.frames_pending_string,
                frames_string: e.frames_string,
                dl_key: e.dl_key,
                dl_created_time: e.dl_created_time,
                dl_status: e.dl_status,
                file_format: e.file_format,
            }
        })
        .collect_vec())
}

pub fn load_dl_image(dl_key: String) -> Result<String> {
    Ok(join_paths(vec![
        get_downloads_dir().as_str(),
        dl_key.as_str(),
    ]))
}

pub fn copy_image_to(src_path: String, to_dir: String) -> Result<()> {
    let name = Path::new(&src_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let ext = image::io::Reader::open(&src_path)?
        .with_guessed_format()?
        .format()
        .with_context(|| anyhow::Error::msg("img format error"))?
        .extensions_str()[0];
    let final_name = format!("{}.{}", name, ext);
    let target = join_paths(vec![to_dir.as_str(), final_name.as_str()]);
    std::fs::copy(src_path.as_str(), target.as_str())?;
    Ok(())
}

pub fn reset_failed_downloads() -> Result<()> {
    block_on(dl_post::reset_failed())
}

pub fn delete_dl_post(dl_key: String) -> Result<()> {
    block_on(dl_post::update_status(dl_key, 3))
}

// get downloads dir form env
pub fn downloads_to() -> Result<String> {
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        Ok(directories::UserDirs::new()
            .unwrap()
            .download_dir()
            .unwrap()
            .join("coco")
            .to_str()
            .unwrap()
            .to_owned())
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err(anyhow::Error::msg("not support os"))
    }
}
