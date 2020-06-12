//mod api;
use std::collections::HashMap;


use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Value, Error};

extern crate dotenv;

use dotenv::dotenv;
use std::env;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
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


use chrono::prelude::*;
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
//#[derive(Debug, Serialize, Deserialize)]
//struct Tag {
//   posts: HashMap<String, String>
//}
//#[derive(Debug, Serialize, Deserialize)]
//struct Tag {
//   name: String,
//}

#[derive(Debug, Deserialize)]
struct Postl {
    slug: String,
    id: String,
    // list no return tags,authors, return html, excerpt,og_image, twitter_image
    // https://ghost.org/docs/api/v3/content/#posts
//    tags: Option<Vec<String>>, 
     tags: Option<Vec<Tag>>,
     //tags: Option<Vec<HashMap<String, String>>>,
//    authors: Option<Vec<String>>,
//    authors will hashmap 
     authors: Option<Vec<Author>>,
//   // status: Option<String>,
   // title: Option<String>,
   // tags:    Vec<String>, 
   // authors: Vec<String>,
    status:  String,
    title:   String,
    //excerpt: Option<String>,
    //cexcerpt: String,
    custom_excerpt: Option<String>,
    updated_at: String,
}
#[derive(Debug, Deserialize)]
struct Author {
    name: String,
    email: String,
}
#[derive(Debug, Deserialize)]
struct Tag {
    name: String,
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

// the op func should only op on the data and queryarg, then use the unify func to make req

pub async fn delete(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let rawkey = makereq().unwrap();
    let key = format!("Ghost {}", rawkey);
    let resp = reqwest::Client::new().request(reqwest::Method::DELETE, format!("https://blog.approachai.com/ghost/api/v3/admin/posts/{}/",id).as_str())
        .header("Authorization", key.as_str())
        .header("Content-Type", "application/json")
        .send()
        .await?;
    println!("delete resp status {}", resp.status());

    Ok(())
      
}
pub async fn list(q1: Vec<Query1>, q2: Vec<Query2>) -> Result<(), Box<dyn std::error::Error>> {
    let rawkey = makereq().unwrap();
    let key = format!("Ghost {}", rawkey);
    
  //  println!("q1 {:?}", q1);
  //  println!("q2 {:?}", q2);

    let resp = reqwest::Client::new().get("https://blog.approachai.com/ghost/api/v3/admin/posts")
        .header("Authorization", key.as_str())
        .header("Content-Type", "application/json")
        .query(&q1)
        .query(&q2)
        // tags some empty then panic
        .send()
        .await?
        .text()
        .await?;

//    println!("list resp status {}", resp.status());
   // println!("list raw: {:#?}", resp);
    let v: Value = serde_json::from_str(&resp)?;

//    println!("list raw: {:?}", v);
    let data: Data = serde_json::from_str(&resp)?;
  // let data: Data = serde_json::from_str(resp.as_ref()).unwrap();
   // println!("list: {:?}", data);
   //println!("test:");
   for post in data.posts {
       println!("slug: {}, id: {}; status:{:?};title: {:?};authors: {:?} ; tags: {:?}|||excerpt: {:?}|||updated_at: {}", post.slug, post.id, post.status, post.title, post.authors.unwrap(), post.tags.unwrap(), post.custom_excerpt,post.updated_at);
   }


    //must annotaon type
   // let v: Value = serde_json::from_str(resp.as_ref())?;
    Ok(())
}
pub async fn edit(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    // need set tags and authors, which not merge,will replace origin post
    let rawkey = makereq().unwrap();
    let key = format!("Ghost {}", rawkey);
    let mut v: Value = serde_json::from_str(data)?;

    let mut m = v.as_object_mut().unwrap();
    println!("m {:?}", m);
    let id = m.remove("id").unwrap();
    // id Value
    //let id = m.remove("id").unwrap().as_str().unwrap();
    let id = id.as_str().unwrap();
    println!("id {}", id);


   // let md = m.remove("md").unwrap();
    // if let Some(md) = m["md"] {
    let md = &m["md"];
    //let md = m.remove("md")?;
    let mut buffer = String::new();
    if md != &json!(null) {
        let p1 = md.as_str().unwrap();
        let mut f = File::open(p1)?;
        println!("f{:?}", f);
        let mdtext = f.read_to_string(&mut buffer)?;
    }
    let mobiledoc;
    if md != &json!(null) {

            mobiledoc = json!({
                                               "version": "0.3.1",
                                               "markups": [],
                                               "atoms": [],
                                               "cards": [[
                                                      "markdown", 
                                                       {
                                                         "cardName": "markdown",
                                                         "markdown": buffer,
                                                       }
                                                       ]],
                                               "sections": [[10,0]]    

         });
         m.insert("mobiledoc".to_string(), Value::String(mobiledoc.to_string()));
    }
//    let tags = m.remove("tags").unwrap();
//    m.insert("tags".to_string(), Value::String(tags.to_string()));
    m.remove("md");


    
    let post_body = json!({
                         "posts": [
                                    m
                                 ],   
                       });
     println!("edit resp json iss : {:#?} ", post_body);
    let resp = reqwest::Client::new().request(reqwest::Method::PUT, format!("https://blog.approachai.com/ghost/api/v3/admin/posts/{}/",id).as_str())
        .header("Authorization", key.as_str())
        .header("Content-Type", "application/json")
        .json(&post_body)
        .send()
        .await?
        .text()
        .await?;
     let v: Value = serde_json::from_str(&resp)?;
     println!("edit resp json iss : {:#?} ", v);

     Ok(())
}
pub async fn post(data: &str) -> Result<(), Box<dyn std::error::Error>> {
    // need md file, tags, status:draft or published
    // upload image or use baidu image
    // custom_excerpt 
    // { md : "./a.md",
    //   tags: [],
    //   status: draft,
    //   custom_excerpt: ""
    //   authors:
    // }
    //   
    let rawkey = makereq().unwrap();
    let key = format!("Ghost {}", rawkey);
    let v: Value = serde_json::from_str(data)?;


    let md = &v["md"];
    let p1 = md.as_str().unwrap();
    let mut f = File::open(p1)?;
    let mut buffer = String::new();
    let mdtext = f.read_to_string(&mut buffer)?;

    let title = &v["title"];
    let mut tags = &v["tags"];
    println!("tags{}", tags);
    let emptytag = &json!([]);
    if tags == &json!(null) {
         //tags = &json!([]);
         tags = emptytag;
    }
    let excerpt = &v["custom_excerpt"];
    
    let defs = &json!("draft");
    let mut status = &v["status"];// json!(null)
    if status == &json!(null) {
        status = defs;
    }


    let mut authors = &v["authors"];
   //  let emptyauthors = &json!([]);
    let emptyauthors = &json!(["bicx@taocloudx.com"]);
    if authors == &json!(null) {
         authors = emptyauthors;
       //  if defaut from env 
       //  authoros = default    
    }

    let mobiledoc = json!({
                                          "version": "0.3.1",
                                          "markups": [],
                                          "atoms": [],
                                          "cards": [[
                                                 "markdown", 
                                                  {
                                                    "cardName": "markdown",
                                                    "markdown": buffer,
                                                  }
                                                  ]],
                                          "sections": [[10,0]]    

    });
    
    let post_body = json!({
                         "posts": [
                                     { 
                                       "title": title, // which is Value
                                       //"tags": ["Note"],
                                       "tags": tags,
                                       // "authors": ["bicx@taocloudx.com"],
                                       "authors": authors,
                                     //  "email": "bicx@taocloudx.com",
                                       "custom_excerpt": excerpt,
                                       "mobiledoc": mobiledoc.to_string(),
                                       "status": status,
      //                                 "mobiledoc":  "{\"version\":\"0.3.1\",\"atoms\":[],\"cards\":[[\"markdown\",{\"cardName\":\"markdown\",\"markdown\":\"head1\"}]],\"markups\":[],\"sections\":[[10,0]]}",         
                                    //   "mobiledoc": {
                                    //       "version": "0.3.1",
                                    //       "markups": [],
                                    //       "atoms": [],
                                    //       "cards": [[
                                    //              "markdown", 
                                    //               {
                                    //                 "cardName": "markdown",
                                    //                 "markdown": "head1"
                                    //               }
                                    //               ]],
                                    //       "sections": [[10,0]]    
                                    //   }

                                     }  
                                     
                                 ],   
                       });

 //   
   println!("post body {}", post_body["posts"][0]);
   println!("post body {}", post_body.to_string());
   println!("post body {:?}", post_body.to_string());
   println!("post body {:#?}", post_body.to_string());
    // return Ok(());   
  
    // const options = {
    //         title: title,
    //         mobiledoc: JSON.stringify({
    //                                   version: '0.3.1',
    //                                   markups: [],
    //                                   atoms: [],
    //                                   cards: [['markdown', {cardName: 'markdown', markdown: Buffer.from(fileContent).toString()}]],
    //                                   sections: [[10, 0]]
    //                               }),
    //         tags: ["Note"],
    //         //authors: ["roidinev@gmail.com"],
    //         authors: ["bicx@taocloudx.com"],
    //         custom_excerpt: "网站、博客文章、论文推荐或评论",
    //         status: 'published'
    // }
    //  pure endpoint format
    //         "mobiledoc": "{\"version\":\"0.3.1\",\"atoms\":[],\"cards\":[],\"markups\":[],\"sections\":[[1,\"p\",[[0,[],0,\"My post content. Work in progress...\"]]]]}",
    //         "mobiledoc": "{\"version\":\"0.3.1\",\"atoms\":[],\"cards\":[[\"markdown\",{\"cardName\":\"markdown\",\"markdown\":\"head1\"}]],\"markups\":[],\"sections\":[[10,0]]}",
    //         \"mobiledoc\":{\"atoms\":[],\"cards\":[[\"markdown\",{\"cardName\":\"markdown\",\"markdown\":\"head1\"}]],\"markups\":[],\"sections\":[[10,0]],\"version\":\"0.3.1\"},

    let resp = reqwest::Client::new().post("https://blog.approachai.com/ghost/api/v3/admin/posts/")
        .header("Authorization", key.as_str())
        .header("Content-Type", "application/json")
        //.body("{"posts":[{"title":"Hello world"}]}")
        .json(&post_body)
        .send() //resposne
        .await?
        .text()
        .await?;

        
     let v: Value = serde_json::from_str(&resp)?;
     println!("post resp json iss : {:#?} ", v);

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
    dotenv().ok();
    let apikey = env::var("API_KEY").unwrap();
    let v: Vec<&str> = apikey.split(':').collect();

    let id = v[0];

    let secrethex = v[1];
    let secret = hex::decode(secrethex.to_owned())?;



    let mut header = Header::default();
    header.kid = Some(id.to_owned());
    let iat = Utc::now().timestamp();
    let exp = iat + 300;

    let aud = "/v3/admin/".to_string();
    let my_claims =
        Claim { iat: iat, exp: exp, aud: "/v3/admin/".to_owned(), };

    let j = serde_json::to_string(&my_claims)?;

    
     let token1 = match encode(&header, &my_claims, &EncodingKey::from_secret(&secret)) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };
    let rpayload =
        Fclaim { iat: iat, exp: exp, aud: "/v3/admin/".to_owned(), };
    let rheader2 =
        Fheader { alg: "HS256".to_string(), typ: "JWT".to_string(), kid: id.to_string(), };
    let payloads = serde_json::to_string(&rpayload)?;
    let header2s = serde_json::to_string(&rheader2)?;
    let payload : Value = serde_json::from_str(&payloads)?; 
    let header2 : Value  = serde_json::from_str(&header2s)?; 


   let token2 =  frank_jwt::encode(header2, &secret, &payload, frank_jwt::Algorithm::HS256).unwrap();
   let decoded2 = frank_jwt::decode(&token2, &secret, frank_jwt::Algorithm::HS256, &frank_jwt::ValidationOptions::default());
   let decoded3 = frank_jwt::validate_signature(&token2, &secret, frank_jwt::Algorithm::HS256)?;

    Ok(token1)

}

