use std::collections::HashMap;

// #[tokio::main]
//pub async fn delete() -> Result<(), Box<dyn std::error::Error>> {
pub async fn delete() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    //println!("{:#?}", resp); //mean
    //Ok(())
    Ok(resp)
  //  return resp; 
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
