extern crate ghost;
extern crate clap;
extern crate tokio;
//extern crate tuple;
//use tuple::*;
//use std::convert::TryInto;
//use std::convert::TryFrom;




#[macro_use]
extern crate maplit;

use clap::{App, AppSettings, Arg, ArgSettings, ArgMatches, SubCommand};
//use ghost::endpoints::{posts, pages, users};
use ghost::endpoints::{dns};
use ghost::framework::{
    apiclient::ApiClient,
    auth::Credentials,
    mock::{MockApiClient, NoopEndpoint},
    response::{ApiFailure, ApiResponse, ApiResult},
    Environment, HttpApiClient, HttpApiClientConfig, OrderDirection,
};
use serde::Serialize;
use std::collections::HashMap;


type SectionFunction<ApiClientType> = fn(&ArgMatches, &ApiClientType);

struct Section<'a, ApiClientType: ApiClient> {// trait bound
    args: Vec<Arg<'a>>,
    description: &'a str,
    function: Option<&'a SectionFunction<ApiClientType>>,
    subcommands: Option<HashMap<&'a str, &'a Section<'a, ApiClientType>>>,
}

fn print_response<T: ApiResult>(response: ApiResponse<T>) {
    match response {
        Ok(success) => println!("Success: {:#?}", success),
        Err(e) => match e {
            ApiFailure::Error(status, errors) => {
                println!("HTTP {}:",status);
                for err in errors.errors {
                    println!("Error {}: {}", err.code, err.message);
                    for (k, v) in err.other {
                        println!("{}: {}", k, v);
                    }
                }
                for (k, v) in errors.other {
                    println!("{}: {}", k, v);
                }
            }
            ApiFailure::Invalid(reqwest_err) => println!("Error: {}", reqwest_err),
        },
    }
}

fn print_response_json<T: ApiResult>(response: ApiResponse<T>)
where 
    T: Serialize,
{
    match response {
        Ok(success) => println!("{}", serde_json::to_string(&success.result).unwrap()),
        Err(e) => match e {
            ApiFailure::Error(status, errors) => {
                println!("HTTP {}:",status);
                for err in errors.errors {
                    println!("Error {}: {}", err.code, err.message);
                    for (k, v) in err.other {
                        println!("{}: {}", k, v);
                    }
                }
                for (k, v) in errors.other {
                    println!("{}: {}", k, v);
                }
            }
            ApiFailure::Invalid(reqwest_err) => println!("Error: {}", reqwest_err),
        },

    }
}

fn dns<ApiClientType: ApiClient>(arg_matches: &ArgMatches, api_client: &ApiClientType) {
    let zone_identifier = arg_matches.value_of("zone_identifier").unwrap();
    let response = api_client.request(&dns::ListDnsRecords {
        zone_identifier,
        params: dns::ListDnsRecordsParams {
            direction: Some(OrderDirection::Ascending),
            ..Default::default()
        },
    });
    print_response(response);
}

fn mock_api<ApiClientType: ApiClient>(_args: &ArgMatches, _api: &ApiClientType) {
    let mock_api = MockApiClient {};
    let endpoint = NoopEndpoint {};
    let _ = mock_api.request(&endpoint);
    println!("ran mock API")
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

        let test = &Section::<HttpApiClient> {// traite not type, must impl for 
            args: vec![Arg::with_name("testcase").required(true)],
            description: "test",
            function: Some(&(dns::<HttpApiClient> as fn(&ArgMatches, &HttpApiClient))),
            subcommands: None::<HashMap<&str, &Section::<HttpApiClient>>>,
        };
        let delete = &Section::<HttpApiClient> {
             args: vec![Arg::with_name("post").required(true),
                        Arg::with_name("postid").short('i').long("id").value_delimiter(",").help("need post id").takes_value(true)
                       ], 
             description: "delete post",
             function: None::<&SectionFunction<HttpApiClient>>,
             subcommands: None::<HashMap<&str, &Section::<HttpApiClient>>>,
         };
        let edit = &Section::<HttpApiClient> {
             args: vec![Arg::with_name("post").required(true),
                        // Arg::with_name("postid").short('i').long("id").value_delimiter(",").help("need post id").takes_value(true)
                        Arg::with_name("data").short('d').long("data").help("edit post body").takes_value(true)
                       ], 
             description: "edit post",
             function: None::<&SectionFunction<HttpApiClient>>,
             subcommands: None::<HashMap<&str, &Section::<HttpApiClient>>>,
         };
         let add = &Section::<HttpApiClient> {
             args: vec![Arg::with_name("post").required(true),
                        Arg::with_name("data").short('d').long("data").help("post body").takes_value(true)
             ], 
             description: "add post",
             function: None::<&SectionFunction<HttpApiClient>>,
             subcommands: None::<HashMap<&str, &Section::<HttpApiClient>>>,
         };
         let list = &Section::<HttpApiClient> {
             args: vec![Arg::with_name("posts").required(true), 
                        Arg::with_name("query").short('q').long("query").value_delimiter("&").value_name("querystring").help("set list query args").takes_value(true)
                        ], 
             description: "list posts",
             function: None::<&SectionFunction<HttpApiClient>>,
             subcommands: None::<HashMap<&str, &Section::<HttpApiClient>>>,
         };
         let gsubcommands = hashmap! {
             "delete" => delete,
             "edit" => edit,
             "add" => add,
             "list" => list,
         };
         let ghost = &Section::<HttpApiClient> {
             // args: vec![Arg::with_name("ghost").required(true)],
             args: vec![],
             description: "op on the ghost blog platform",
             function: None::<&SectionFunction<HttpApiClient>>,
             subcommands: Some(gsubcommands), 
         };
    let sections = hashmap! {
        "test" => test,
        "ghost" => ghost,
    };


    let mut cli = App::new("Share")
        .version("1.0")
        .author("baul <roidinev@gmail.com>")
        .about("The hub for blogger,publisher,indieWeber ")
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output") 
                .help("Sets an optional output file")
                .index(1),
        )
        .arg(
            Arg::with_name("debug")
                .short('d')
                .multiple(true)
                .help("Turn debugging information on"),
        )
        .setting(AppSettings::ArgRequiredElseHelp);

   for (section_name, section) in sections.iter() {
       let mut subcommand = App::new(section_name.to_string()).about(section.description);
       for arg in &section.args {
           subcommand = subcommand.arg(arg);
       }
   //    match section.function {
   //        // Some(f) => println!("have action,level end:{}",section_name), 
   //        Some(f) => (), //or {},
   //        None => { 
   //            for (section_name, section) in section.subcommands.as_ref().unwrap().iter() {
   //                let mut ssubcommand = App::new(section_name.to_owned()).about(section.description);
   //                for arg in &section.args {
   //                    ssubcommand = ssubcommand.arg(arg);
   //                }
   //                subcommand = subcommand.subcommand(ssubcommand);
   //         
   //            }

   //        },
   //    }
       if let None = section.function {
       //if section.function.is_none() {
      // if section.function.is_none() {
       // if section.function.unwrap_or() {  
               for (section_name, section) in section.subcommands.as_ref().unwrap().iter() {
                   let mut ssubcommand = App::new(section_name.to_owned()).about(section.description);
                   for arg in &section.args {
                       ssubcommand = ssubcommand.arg(arg);
                   }
                   subcommand = subcommand.subcommand(ssubcommand);
            
               }

       }

    //   need return pass??
    //    section.function.unwrap_or_else(|| -> &App 
    //    {
    //           for (section_name, section) in section.subcommands.as_ref().unwrap().iter() {
    //               let mut ssubcommand = App::new(section_name.to_owned()).about(section.description);
    //               for arg in &section.args {
    //                   ssubcommand = ssubcommand.arg(arg);
    //               }
    //               subcommand = subcommand.subcommand(ssubcommand);

    //        
    //           }
    //           //&subcommand
    //    }  

    //   ); 
       // loop the nest subcommand
       cli = cli.subcommand(subcommand);
       // first level test have function,no subcomands
       // first level ghost no function, have subcommands
   }




    let matches = cli.get_matches();

    // matches.subcommand_matches independent match, less command, maybe conflict
    // cloudflare use this for match the 1-level subcommand.
    //
    // cloudflare then get email/key/token and create credentials then new api_client 
    // according the matcheds to loop ,using the function suppiled, but now we use the 
    // matches.subcommand to precise match and do api by call lib function not using 
    // the sections function mechnaics
    //
    // another using builder pattern?
    match matches.subcommand() {
        ("ghost", Some(ghost_matches)) => {
            // Now we have a reference to ghost's matches
            match ghost_matches.subcommand() {
                // since so many subcommands ,we can use for loop to reduce code 
                // ref https://github.com/cloudflare/cloudflare-rs/blob/master/cloudflare-examples/src/main.rs
                ("add", Some(add_matches)) => {
                    println!("to post posts/pages { }", add_matches.value_of("post").unwrap());
                    println!("to post posts/pages's body: {:?}", add_matches.value_of("data").unwrap());
                    println!("to post posts/pages's body: {}", add_matches.value_of("data").unwrap());
                    let data = add_matches.value_of("data").unwrap();


                    // call lib ghost
                    // why need await ,only no use some await
                    let resp = ghost::post(data).await?;
                    println!("post blog {:#?}", resp); //mean
                }
                ("edit", Some(edit_matches)) => {
                    let data = edit_matches.value_of("data").unwrap();
                    println!("to edit posts/page: {}", data);
                    let resp = ghost::edit(data).await?;
                    println!("edit blog {:#?}", resp); //mean
                }
                ("delete", Some(delete_matches)) => {
                    // Now we have a reference to delete's matches
                    if let Some(id) = delete_matches.values_of("postid") {
                       for var in id {
                           println!("idd{}", var);
                           let resp = ghost::delete(var.to_string()).await?;
                       }
                  }

                }
                ("list", Some(list_matches)) => {

                    type Query1 = (String, String);
                    type Query2 = (String,);
                    let mut q1: Vec<Query1> = Vec::new();
                    let mut q2: Vec<Query2> = Vec::new();
                    if let Some(q) = list_matches.values_of("query") {
                        let v = q.collect::<Vec<_>>() ;
                        for arg in &v {
                            if arg.contains("=") == true {
                                let mut iter = arg.split("=");
                                let tuple : Query1 = (iter.next().unwrap().to_string(), iter.next().unwrap().to_string());
                                q1.push(tuple);
                                continue;

                            }
                            let tuple: Query2 = (arg.to_string(),);
                            q2.push(tuple);
                        }
                     //   for arg in &v {
                     //       let tokens:Vec<&str>= arg.split(":").collect();
                     //      // let a: [&str; 2] = tokens.try_into()?;
                     //       //let a: [&str; 2] = tokens.into()?;
                     //      // let slice = &tokens[..2];
                     //      // let mut a = [&str; 2];
                     //      // a.copy_from_slice(slice);
                     //       //let t: T2<_,_> = tokens.into();
                     //       // let t: T2<_,_> = a.into();
                     //       //println!("tokens:{:#?}", tokens.into());
                     //       //println!("tokens:{:#?}", t);

                     //   }
                        
                    }
                        let resp = ghost::list(q1, q2).await?;
                }
                ("", None) => println!("No ghost subcommand was used"), // If no subcommand was usd it'll match the tuple ("", None)
                _ => unreachable!(),
            }
        }
        ("", None) => println!("No main subcommand was used"), // If no subcommand was usd it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }


//    // You can see how many times a particular flag or argument occurred
//    // Note, only flags can have multiple occurrences
//    match matches.occurrences_of("debuggg") {
//        0 => println!("Debug mode is off"),
//        1 => println!("Debug mode is kind of on"),
//        2 => println!("Debug mode is on"),
//        3 | _ => println!("Don't be crazy"),
//    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
//    println!(matches)
    if let Some(ref matches) = matches.subcommand_matches("test") {
        // "$ myapp test" was run
        if matches.is_present("list") {
            // "$ myapp test -l" was run
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }


    // Continued program logic goes here...
    Ok(())
}


