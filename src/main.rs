use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;
use windows::Storage::Streams::IRandomAccessStreamReference;
use windows::Storage::Streams::IRandomAccessStreamWithContentType;
use windows::Storage::Streams::{DataReader, DataReaderLoadOperation};
use std::fs::File;
use tokio::runtime::Runtime;
use std::io::Write;

fn main() {
    let session_manager = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().unwrap().get().unwrap();
    let current_session = session_manager.GetCurrentSession().unwrap();
    let media_properties = current_session.TryGetMediaPropertiesAsync().unwrap().get().unwrap();
    let thumbnail_reference: IRandomAccessStreamReference = media_properties.Thumbnail().unwrap();
    let thumbnail_stream: IRandomAccessStreamWithContentType = thumbnail_reference.OpenReadAsync().unwrap().get().unwrap();
    let data_input_stream = thumbnail_stream.GetInputStreamAt(0).unwrap();

   // Create a DataReader for the DataInputStream:
    let data_reader = DataReader::CreateDataReader(&data_input_stream).unwrap();

    let buffer_size = data_reader.UnconsumedBufferLength().unwrap();

   // Load the data from the stream into the DataReader:
    let load_operation: DataReaderLoadOperation = data_reader.LoadAsync(buffer_size as u32).unwrap();
    load_operation.GetResults().unwrap();

    // Read the data from the DataReader into a Vec<u8>:
    let mut data: Vec<u8> = vec![0; buffer_size as usize];
    data_reader.ReadBytes(&mut data).unwrap();

    // Write the data to the file:
    let mut file = File::create("thumbnail.jpg").unwrap();
    file.write_all(&data).unwrap();
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // ... rest of your code here ...

        // Load the data from the stream into the DataReader:
        let load_operation: DataReaderLoadOperation = data_reader.LoadAsync(buffer_size as u32).unwrap();
        load_operation.await.unwrap();

        load_operation.GetResults().unwrap();

        // ... rest of your code here ...
    });
}