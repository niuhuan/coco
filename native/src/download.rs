use std::path::Path;
use std::time;

use crate::api::down_image;
use crate::database::active::dl_post;
use crate::database::active::dl_post::{
    delete_download_info, first_need_delete_post, first_not_result_post,
};
use crate::{get_downloads_dir, join_paths};

pub(crate) async fn download_demon() {
    loop {
        loop {
            if let Some(info) = first_need_delete_post().await.unwrap() {
                let local_path = info.dl_key.clone();
                let abs_path = join_paths(vec![get_downloads_dir().as_str(), &local_path]);
                if Path::new(&abs_path).exists() {
                    let _ = tokio::fs::remove_file(&abs_path).await;
                }
                let _ = delete_download_info(info.dl_key).await;
                // todo notify
            } else {
                break;
            }
        }
        if let Some(info) = first_not_result_post().await.unwrap() {
            down_post(info).await;
        } else {
            tokio::time::sleep(time::Duration::from_secs(3)).await;
        }
    }
}

async fn down_post(model: dl_post::Model) {
    match down_image(model.file_url.clone()).await {
        Ok((bytes, format)) => {
            let local_path = format!("{}.{}", model.dl_key, format);
            let abs_path = join_paths(vec![get_downloads_dir().as_str(), &local_path]);
            tokio::fs::write(&abs_path, &bytes).await.unwrap(); // todo: this is file system error, need stop download
            let _ = delete_download_info(model.dl_key).await;
        }
        Err(err) => {
            println!("{:?}", err);
            dl_post::update_status(model.dl_key, 2).await.unwrap();
        }
    }
}
