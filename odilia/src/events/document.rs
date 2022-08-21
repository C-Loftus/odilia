use zbus::zvariant::ObjectPath;
use crate::state;
use atspi::events::Event;

pub async fn load_complete(event: Event) -> eyre::Result<()> {
    let dest = event.sender()?.unwrap();
    let cache = state::build_cache(
        dest, ObjectPath::try_from("/org/a11y/atspi/cache".to_string())?
    ).await?;
    let entire_cache = cache.get_items().await?;
    let write_by_id_m = state::by_id_write().await;
    let mut write_by_id = write_by_id_m.lock().await;
    for item in entire_cache {
        // defined in xml/Cache.xml
        let path = item.0.1.to_string();
        let dest = item.0.0.to_string();
        if let Some(id) = path.split('/').next_back() {
            if let Ok(uid) = id.parse::<u32>() {
                write_by_id.insert(uid, (path, dest));
                tracing::debug!("ID: {:?}", uid);
            }
        }
    }
    write_by_id.refresh();
    Ok(())
}

pub async fn dispatch(event: Event) -> eyre::Result<()> {
    // Dispatch based on member
    if let Some(member) = event.member() {
        match member.as_str() {
            "LoadComplete" => load_complete(event).await?,
            member => tracing::debug!(member, "Ignoring event with unknown member"),
        }
    }
    Ok(())
}
