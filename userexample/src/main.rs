use google_cloud_storage::{Client, Result};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new()).unwrap();
    let client = Client::new().await?;
    let mut bytes: &[u8] = "testing upload".as_bytes();

    // upload an object from memory
    let resp = client
        .objects_service()
        .insert("codyoss-workspace", Default::default())
        .name("rust-test.txt")
        .media_content_type("text/plain; charset=utf-8")
        .upload(&mut bytes)
        .await?;

    info!("Uploaded from memory: {:?}", resp);

    // upload an object from a file
    let mut file = tokio::fs::File::open(
        "/Users/codyoss/oss/google-cloud-rust/userexample/resources/upload-me.txt",
    )
    .await
    .unwrap();

    client
        .objects_service()
        .insert("codyoss-workspace", Default::default())
        .name("rust-test-2.txt")
        .media_content_type("text/plain; charset=utf-8")
        .upload(&mut file)
        .await?;
    info!("Uploaded a file!");

    // Pull files its metadata
    let resp = client
        .objects_service()
        .get("codyoss-workspace", "rust-test.txt")
        .execute()
        .await?;
    info!("Updated metadata at: {:?}", resp.updated.unwrap());

    // Download the contents
    let mut buf: Vec<u8> = vec![];
    client
        .objects_service()
        .get("codyoss-workspace", "rust-test.txt")
        .download(&mut buf)
        .await?;
    info!("File contents: {}", String::from_utf8(buf).unwrap());

    // Iterate over a bucket (kinda)
    let resp = client
        .objects_service()
        .list("codyoss-workspace")
        .execute()
        .await?;
    for item in resp.items.unwrap() {
        info!("- {}", item.name.unwrap());
    }

    Ok(())
}
