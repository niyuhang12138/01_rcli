//! Process the csv file and delete the corresponding format
use anyhow::Result;
use csv::Reader;
use serde_json::Value;

use crate::cli::OutputFormat;

// use serde::{Deserialize, Serialize};
// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
// pub struct Player {
//     // #[serde(rename = "Name")]
//     name: String,
//     // #[serde(rename = "Position")]
//     position: String,
//     #[serde(rename = "DOB")]
//     dob: String,
//     // #[serde(rename = "Nationality")]
//     nationality: String,
//     #[serde(rename = "Kit Number")]
//     kit: u8,
// }

/// Process the csv file and delete the corresponding format
pub async fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    // use csv reader to read csv file
    let mut reader = Reader::from_path(input)?;

    // use vector to store csv file content
    let mut ret = Vec::with_capacity(128);

    // get csv file headers
    let headers = reader.headers()?.clone();

    for result in reader.records() {
        // get csv file event line
        let record = result?;
        // match the header to the field, collect to a vector
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    // converts the result to the corresponding format
    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => "This format is currently not supported".to_string(),
    };

    // output the result to the corresponding file
    tokio::fs::write(output, content).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[tokio::test]
    async fn test_process_csv() -> Result<()> {
        process_csv(
            "fixtures/juventus.csv",
            "output.json".into(),
            OutputFormat::Json,
        )
        .await?;

        process_csv(
            "fixtures/juventus.csv",
            "output.yaml".into(),
            OutputFormat::Yaml,
        )
        .await?;

        Ok(())
    }
}
