use serde_json::{json, Value};
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn translate(
    text: &str,
    from: &str,
    to: &str,
    needs: HashMap<String, String>,
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;
    const URL: &str = "https://tatoeba.org/eng/api_v0/search";
    let res: Value = client
        .get(URL)
        .query(&[
            ("query", text),
            ("from", from),
            ("to", to),
            ("has_audio", "true"),
            ("sort", "relevance"),
        ])
        .send()?
        .json()?;

    fn parse_result(res: Value) -> Option<Value> {
        let results = res.as_object()?.get("results")?.as_array()?;
        let mut sentence_list = Vec::new();
        for i in results {
            let source = i.as_object()?.get("text")?.as_str()?;
            let mut target = String::new();
            let translations = i.as_object()?.get("translations")?.as_array()?;
            for j in translations {
                for k in j.as_array()? {
                    target.push_str(k.as_object()?.get("text")?.as_str()?);
                    target.push_str("\n");
                }
            }
            let sentence = json!({"source":source,"target":target});
            sentence_list.push(sentence);
        }
        Some(json!({"sentence":sentence_list}))
    }
    if let Some(result) = parse_result(res) {
        return Ok(result);
    } else {
        return Err("Response Parse Error".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let needs = HashMap::new();
        let result = translate("你好 世界！", "", "eng", needs).unwrap();
        println!("{result:?}");
    }
}
