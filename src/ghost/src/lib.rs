mod api;
// extern crate chrono
use std::collections::HashMap;

//use chrono::prelude::*;

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
//use serde_json::{Result, Value};
// #[macro_use]
//extern crate serde_json;
use serde_json::json;
use serde_json::{Value, Error};



use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
//extern crate hex;
//use hex as Hex;
extern crate frank_jwt;


extern crate chrono;
extern crate reqwest;
#[macro_use]
extern crate serde;
extern crate serde_json;
extern crate serde_qs;
extern crate url;
pub mod endpoints;
pub mod framework;

//#macro_use] extern crate serde_json;
//use frank_jwt::{Algorithm, encode, decode};
//use frank_jwt::{Algorithm, encode, decode};


//#[derive(Debug, Serialize, Deserialize)]
//struct Claims {
//    #[serde(with = "jwt_numeric_date")]
//    iat: DateTime<Utc>,
//    #[serde(with = "jwt_numeric_date")]
//    exp: DateTime<Utc>,
//    aud: String,
//}

#[derive(Debug, Serialize, Deserialize)]
struct Claim {
    iat: i64,
    exp: i64,
    aud: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Fclaim {
    iat: i64,
    exp: i64,
    aud: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Fheader {
    alg: String,
    typ: String,
    kid: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct Post {
   posts: Vec<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct Postl {
    slug: String,
    id: String,
}

#[derive(Debug, Deserialize)]
struct Data {
    posts: Vec<Postl>,
}
type Query1 = (String, String);
type Query2 = (String,);

mod jwt_numeric_date {
    //! Custom serialization of DateTime<Utc> to conform with the JWT spec (RFC 7519 section 2, "Numeric Date")
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    /// Serializes a DateTime<Utc> to a Unix timestamp (milliseconds since 1970/1/1T00:00:00T)
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let timestamp = date.timestamp();
        serializer.serialize_i64(timestamp)
    }

    /// Attempts to deserialize an i64 and use as a Unix timestamp
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single() // If there are multiple or no valid DateTimes from timestamp, return None
            .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
    }

    #[cfg(test)]
    mod tests {
        const EXPECTED_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJDdXN0b20gRGF0ZVRpbWUgc2VyL2RlIiwiaWF0IjowLCJleHAiOjMyNTAzNjgwMDAwfQ.RTgha0S53MjPC2pMA4e2oMzaBxSY3DMjiYR2qFfV55A";

        use super::super::{Claims, SECRET};

        #[test]
        fn round_trip() {
            let sub = "Custom DateTime ser/de".to_string();
            let iat = Utc.timestamp(0, 0); //sinc sec, nsecs 0
            let exp = Utc.timestamp(32503680000, 0);

            let claims = Claims { sub: sub.clone(), iat, exp };

            let token =
                encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET.as_ref()))
                    .expect("Failed to encode claims");

            assert_eq!(&token, EXPECTED_TOKEN);

            let decoded = decode::<Claims>(
                &token,
                &DecodingKey::from_secret(SECRET.as_ref()),
                &Validation::default(),
            )
            .expect("Failed to decode token");

            assert_eq!(decoded.claims, claims);
        }

        #[test]
        fn should_fail_on_invalid_timestamp() {
            // A token with the expiry of i64::MAX + 1
            let overflow_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJDdXN0b20gRGF0ZVRpbWUgc2VyL2RlIiwiaWF0IjowLCJleHAiOjkyMjMzNzIwMzY4NTQ3NzYwMDB9.G2PKreA27U8_xOwuIeCYXacFYeR46f9FyENIZfCrvEc";

            let decode_result =
                decode::<Claims>(&overflow_token, SECRET.as_ref(), &Validation::default());

            assert!(decode_result.is_err());
        }
    }
}



// #[tokio::main]
//pub async fn delete() -> Result<(), Box<dyn std::error::Error>> {
pub async fn delete() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    //println!("{:#?}", resp); //mean is result enum
    //Ok(())
    Ok(resp)
}
//pub async fn list() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
pub async fn list(q1: Vec<Query1>, q2: Vec<Query2>) -> Result<(), Box<dyn std::error::Error>> {
    let rawkey = makereq().unwrap();
    let key = format!("Ghost {}", rawkey);
    println!("query stirng{:?} {:?}",q1, q2 );
    
    println!("key is : {}",key);
    let resp = reqwest::Client::new().get("https://blog.approachai.com/ghost/api/v3/admin/posts")
        .header("Authorization", key.as_str())
        .header("Content-Type", "application/json")
       // .query(&[("fields","title,url,id,slug,status"),("limit","500"),("page", "1"),("status","draft")])
        .query(&q1)
        .query(&q2)
        // tags some empty then panic
        .send()
        .await?
        .text()
        .await?;

    // println!("slug {:#?}", serde_json::from_str::<Value>(&resp));
    let v: Value = serde_json::from_str(&resp)?;
    println!("posts meta: {}", v["meta"]);
    //println!("slug {}", v["posts"][0]["slug"]);
    //let v["posts"] = ();
    println!("is{}", v["posts"].is_array());
    //for slug in v["posts"].as_array().iter() {
    //for slug in v["posts"].into_iter() {
    //for slug in v["posts"] {
    //   println!("slug: {}", slug["slug"]);
    //
    //}
    let data: Data = serde_json::from_str(&resp).unwrap();
    for post in data.posts {
       println!("slug: {}, id: {}", post.slug, post.id);
    }


    // have json value remine why
    //println!("slug {:#?}", serde_json::from_str::<Value>(&resp).unwrap().to_string());

   // println!("list resp {:#?}", resp.json()); //mean is result enum
    //let v: Value = serde_json::from_str(resp).unwrap()?;
    //let v = serde_json::from_str(resp.as_ref()).unwrap()?;
    let v = serde_json::from_str(resp.as_ref())?;
   // println!("list resp {}", v);                                    
    Ok(())
    //Ok(resp)
    //Ok(resp)
}
pub async fn post() -> Result<(), Box<dyn std::error::Error>> {
    let rawkey = makereq().unwrap();
    let key = format!("Ghost {}", rawkey);
   // println!("Author: {}", key);
    //let () = key; String
    
//    println!("key is : {}",key);
//    let resp = reqwest::Client::new().post("https://blog.approachai.com/ghost/api/v3/admin/posts")
//        .header("Authorization", key.as_str())
//        //.header("Authorization", key)
//        .header("Content-Type", "application/json")
//        .body("{"posts":[{"title":"Hello world"}]}")
//        .send()
//        .await?
//        //.json()
//        .text()
//        .await?;
//
//    println!("post resp {:#?}", resp); //mean is result enum
  let post_body = json!({
                         "posts": [
                                     { 
                                       "title": "test hatitel"
                                     }  
                                 ],   
                       });

   // let mut map = HashMap::new();
   // map.insert("title", "test title");
    //then json(&map);
//   let resp = reqwest::Client::new().post("https://blog.approachai.com/ghost/api/v3/admin/posts")
//       .header("Authorization", key.as_str())
//       .header("Content-Type", "application/json")
//       //.body("{"posts":[{"title":"Hello world"}]}")
//       .json(&post_body)
//       //.body("aa")
//       .send() //resposne
//       .await?;


   let resp = reqwest::Client::new().post("https://blog.approachai.com/ghost/api/v3/admin/posts/")
       .header("Authorization", key.as_str())
       .header("Content-Type", "application/json")
       //.body("{"posts":[{"title":"Hello world"}]}")
       .json(&post_body)
       .send() //resposne
       .await?
       .text()
       //.json()
       .await?;

       
   // println!("post resp is :  {:#?}", resp);
    let v: Value = serde_json::from_str(&resp)?;
    println!("post resp json iss : {:#?} ", v);
    // invalid type: map, expected a string
    // println!("post resp json is :  {:?}", serde_json::from_str(&resp)?);

//for resopne print 
//    match resp.status() {
//        reqwest::StatusCode::OK => println!("success"),
//        s => println!("status: {:?}",s),
//     };

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
fn makereq() -> Result<String, Box<dyn std::error::Error>> {
    let apikey = "***REMOVED***"; 
    let v: Vec<&str> = apikey.split(':').collect();

    let id = v[0];

    let secrethex = v[1];
   // println!("id {}, secret {:?}", id, secrethex); 
    let secret = hex::decode(secrethex.to_owned())?;



    let mut header = Header::default();
    //header.kid = Some("signing_key".to_owned());
    header.kid = Some(id.to_owned());
    //header.kid = Some(id.to_string());
     // println!("{}",header);
    // println!("{:#?}",header);
     //() = id; //&str
     //() = id.to_owned(); //String
//    header.alg = Algorithm::HS256;
 //   header.typ = "JWT".to_owned();

 //   let iat = Utc::now();
 //   //let exp = iat + chrono::Duration::days(1);
 //   // let exp = iat + 5 * 60;
 //   let exp = iat + chrono::Duration::minutes(5);

 //   let aud = "/v3/admin/".to_string();
 //   let my_claims =
 //       //Claims { iat, exp, aud: "/v3/admin".to_owned() };
 //       Claims { iat, exp, aud: aud.clone() };
    let iat = Utc::now().timestamp();
    let exp = iat + 300;

    let aud = "/v3/admin/".to_string();
    let my_claims =
        Claim { iat: iat, exp: exp, aud: "/v3/admin/".to_owned(), };

//    println!("my_claims: {:#?}", my_claims);
    let j = serde_json::to_string(&my_claims)?;
//    println!("my_claims to_string {}", j);
////  to test weather encode same as the bash script ----
// and in src/jsonwebtoken/src/serialization.rs to simulat the json as bash
//    let data = r#"
//    { "iat": 1584504692,
//      "exp": 1584504992,
//      "aud": "/v3/admin/"
//    }"#;
//    let my_claims : Claim = serde_json::from_str(data)?;
//    println!("my_claims from str : {:#?}", my_claims);
//  to test weather encode same as the bash script ----

    
//    //as_bytes() or b''
//    // HS256 mean HMAC,not base64.
//    // from_secret(&[u8])
//    //let token = match encode(&header, &my_claims, &EncodingKey::from_secret(secret)) {
//    //let token = match encode(&header, &my_claims, &EncodingKey::from_secret(&secret.as_bytes())) {
//    //base64sectet.
// we use hex decode!!!
     // let token1 = match encode(&header, &my_claims, &EncodingKey::from_secret(secret.as_bytes())) {
     // for hex stirng to decode
     let token1 = match encode(&header, &my_claims, &EncodingKey::from_secret(&secret)) {
    //let token1 = match encode(&header, &my_claims, &EncodingKey::from_base64_secret(secret.as_ref()).unwrap()) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };
    // since the sign process have change ,the validation have defect
     //       let decoded = decode::<Claim>(
     //           &token1,
     //           &DecodingKey::from_secret(secret.as_ref()),
     //           //&Validation::default(),
     //           &Validation::new(Algorithm::HS256),
     //       )
     //       .expect("Failed to decode token");

     //       //assert_eq!(decoded.claims, my_claims);

     //       println!("decoded {:#?}", decoded);

    //println!("jwt1 token : {:#?}", token1);
//  prank_jwt test-------    
//  //cannot keep order why
//   let mut header2 = json!({
//                "alg": "HS256",
//                "typ": "JWT",
//                //"kid": id.as_str(),
//                "kid": id,
//   });
//   println!("jwt2 header2: {:#?}", header2);
//   let mut payload = json!({
//       "aud": aud.clone(),
//       "exp": exp,
//       "iat": iat,
//   });
//   println!("jwt2 payload: {:#?}", payload);
    let rpayload =
        Fclaim { iat: iat, exp: exp, aud: "/v3/admin/".to_owned(), };
    let rheader2 =
        Fheader { alg: "HS256".to_string(), typ: "JWT".to_string(), kid: id.to_string(), };
    let payloads = serde_json::to_string(&rpayload)?;
    let header2s = serde_json::to_string(&rheader2)?;
    let payload : Value = serde_json::from_str(&payloads)?; 
    let header2 : Value  = serde_json::from_str(&header2s)?; 


   //let token2 =  frank_jwt::encode(header2, &secret.to_owned(), &payload, frank_jwt::Algorithm::HS256).unwrap();
     // for hex stirng to decode
   //let token2 =  frank_jwt::encode(header2, &secret.to_string(), &payload, frank_jwt::Algorithm::HS256).unwrap();
   let token2 =  frank_jwt::encode(header2, &secret, &payload, frank_jwt::Algorithm::HS256).unwrap();
  // println!("jwt2 token : {:#?}", token2);
  // let (header, payload) = frank_jwt::decode(&token2, &secret, frank_jwt::Algorithm::HS256, &ValidationOptions::default());
   let decoded2 = frank_jwt::decode(&token2, &secret, frank_jwt::Algorithm::HS256, &frank_jwt::ValidationOptions::default());
   let decoded3 = frank_jwt::validate_signature(&token2, &secret, frank_jwt::Algorithm::HS256)?;
   //println!("decoded2 {:#?}", decoded2);
   //println!("decoded3 {:#?}", decoded3); //true


    Ok(token1)
    //Ok(token2)

}

