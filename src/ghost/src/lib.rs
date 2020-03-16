use std::collections::HashMap;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp =reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>)
        .await?;
//    println("{:#?}", resp);
//   Ok(())
    return resp; 
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
