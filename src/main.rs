use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

fn main() {
    let session_manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().expect("Failed to get session manager").get().unwrap();
    let current_session = session_manager.GetCurrentSession().unwrap();

    let media_properties = current_session.TryGetMediaPropertiesAsync().unwrap().get().unwrap();

    let title = media_properties.Title().unwrap();
    let artist = media_properties.Artist().unwrap();
    let album = media_properties.AlbumTitle().unwrap();

    println!("Title: {}", title);
    println!("Artist: {}", artist);
    println!("Album: {}", album);

}