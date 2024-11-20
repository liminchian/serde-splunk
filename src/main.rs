use anyhow::Result;

use serde_splunk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let xml = r#"
        <response>
          <dict>
            <key name="-12h">2011-07-06T21:54:23.000-07:00</key>
            <key name="-24h">2011-07-06T09:54:23.000-07:00</key>
          </dict>
        </response>
        "#;

    let resp = quick_xml::de::from_str::<TimeParser>(xml)?;

    println!("{:?}", resp);

    Ok(())
}
