#[cfg(not(feature = "tokio"))]
fn main() {

    use dataforge::write_df_message_sync;

    let data = r#"
        {
            "type": "point"
        }"#;

        // Parse the string of data into serde_json::Value.
        let meta: serde_json::Value = serde_json::from_str(data).unwrap();

        let mut stream = vec![];
        write_df_message_sync(&mut stream, meta, None).unwrap();

        println!("{:?}", String::from_utf8_lossy(&stream))
}

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() {

    use dataforge::write_df_message;

    let data = r#"
        {
            "type": "point"
        }"#;

        // Parse the string of data into serde_json::Value.
        let meta: serde_json::Value = serde_json::from_str(data).unwrap();

        let mut stream = vec![];
        write_df_message(&mut stream, meta, None).await.unwrap();

        println!("{:?}", String::from_utf8_lossy(&stream))
}