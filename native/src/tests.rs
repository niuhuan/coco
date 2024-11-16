#[test]
fn test1() -> anyhow::Result<()> {
    crate::api::init("/tmp/coco".to_owned());
    crate::api::set_host("https://konachan.net".to_owned());
    println!("{}", serde_json::to_string(&crate::api::tag_summary()?)?);
    Ok(())
}

#[test]
fn test2() -> anyhow::Result<()> {
    crate::api::init("/tmp/coco".to_owned());
    crate::api::set_host("https://konachan.net".to_owned());
    println!(
        "{}",
        serde_json::to_string(&crate::api::load_posts("".to_string(), 1)?)?
    );
    Ok(())
}
