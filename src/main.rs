use std::time::Duration;

use aws_credential_types::{provider::SharedCredentialsProvider, Credentials};
use aws_sdk_s3 as s3;
use aws_smithy_client::{http_connector::ConnectorSettings, hyper_ext};
use aws_types::{region::Region, SdkConfig};
use s3::primitives::ByteStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let https_connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_webpki_roots().https_or_http()
        .enable_http1()
        .enable_http2()
        .build();
    let smithy_connector = hyper_ext::Adapter::builder()
        // Optionally set things like timeouts as well
        .connector_settings(
            ConnectorSettings::builder()
                .connect_timeout(Duration::from_secs(5))
                .build(),
        )
        .build(https_connector);
    let shared_config = SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(Credentials::new(
            "Q3AM3UQ867SPQQA43P2F",
            "zuf+tfteSlswRu7BJ86wekitnifILbZam1KYY3TG",
            None,
            None,
            "Static",
        )))
        .http_connector(smithy_connector)
        .endpoint_url("https://play.min.io:9000")
        .region(Region::new("us-east-1"))
        .build();
    let s3_config_builder = aws_sdk_s3::config::Builder::from(&shared_config);
    let client = aws_sdk_s3::Client::from_conf(s3_config_builder.build());
    let obj_list = client.list_buckets();

    let list = obj_list.send().await?;
    let b = list.buckets().unwrap();
    println!("{:?}", b.len());
    println!("{:?}", b[0].name);

    let content = ByteStream::read_from()
        .path(r"d:\Desktop\199_S.jpg")
        // Artificially limit the buffer size to ensure the file has multiple
        // progress steps.
        .buffer_size(2048)
        .build()
        .await?;

    let request = client
        .put_object()
        .bucket("00bucket")
        .key("demo.jpg")
        .body(content);
    println!("{:?}", request);
    let customized = request.customize().await?;

    let out = customized.send().await;

    match out {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e)
        }
    }

    anyhow::Ok(())
}
