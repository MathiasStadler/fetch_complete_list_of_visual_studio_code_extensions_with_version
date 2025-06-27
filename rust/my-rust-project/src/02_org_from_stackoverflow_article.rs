use reqwest::blocking::{Client};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use serde::{Deserialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
struct Publisher {
    publisherName: String,
}

#[derive(Debug, Deserialize)]
struct Statistic {
    statisticName: String,
    value: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct Version {
    version: String,
}

#[derive(Debug, Deserialize)]
struct Extension {
    extensionName: String,
    versions: Vec<Version>,
    statistics: Vec<Statistic>,
    publisher: Publisher,
}

#[derive(Debug, Deserialize)]
struct ResultItem {
    extensions: Vec<Extension>,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    results: Vec<ResultItem>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .build()?;

    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/json; charset=utf-8; api-version=7.2-preview.1"),
    );

    let page_size = 100;
    let max_page = 1; // You can increase this for more results
    let mut page = 1;

    loop {
        let flags = 0x1 | 0x2 | 0x4 | 0x8 | 0x10 | 0x40 | 0x80 | 0x100 | 0x8000;

        let body = json!({
            "filters": [{
                "criteria": [{
                    "filterType": 8,
                    "value": "Microsoft.VisualStudio.Code"
                }],
                "pageNumber": page,
                "pageSize": page_size,
                "sortBy": 0,
                "sortOrder": 0
            }],
            "assetTypes": [],
            "flags": flags
        });

        let resp = client
            .post("https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery")
            .headers(headers.clone())
            .json(&body)
            .send()?
            .json::<ApiResponse>()?;

        let extensions = &resp.results[0].extensions;
        for ext in extensions {
            let extension_name = &ext.extensionName;
            let extension_description = &ext.extensionName;
            let publisher = &ext.publisher.publisherName;
            let statistics: std::collections::HashMap<_, _> = ext.statistics.iter()
                .map(|s| (s.statisticName.as_str(), &s.value))
                .collect();

            for version in &ext.versions {
                let extension_version = &version.version;
                let download_url = format!(
                    "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{}/vsextensions/{}/{}/vspackage",
                    publisher, extension_name, extension_version
                );
                let installs = statistics.get("install").unwrap_or(&&serde_json::Value::Null);
                println!(
                    "{}\t{}\t{}\t{}\t{}",
                    extension_name, extension_description, extension_version, download_url, installs
                );
            }
        }

        if extensions.len() != page_size {
            break;
        }
        page += 1;
        if page > max_page {
            break;
        }
    }

    Ok(())
}