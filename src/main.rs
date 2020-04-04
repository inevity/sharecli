extern crate ghost;
extern crate clap;
extern crate tokio;


#[macro_use]
extern crate maplit;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
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
             args: vec![Arg::with_name("post").required(true)], 
             description: "delete post",
             function: None::<&SectionFunction<HttpApiClient>>,
             subcommands: None::<HashMap<&str, &Section::<HttpApiClient>>>,
         };
         let add = &Section::<HttpApiClient> {
             args: vec![Arg::with_name("post").required(true)], 
             description: "add post",
             function: None::<&SectionFunction<HttpApiClient>>,
             subcommands: None::<HashMap<&str, &Section::<HttpApiClient>>>,
         };
         let list = &Section::<HttpApiClient> {
             args: vec![Arg::with_name("posts").required(true)], 
             description: "list posts",
             function: None::<&SectionFunction<HttpApiClient>>,
             subcommands: None::<HashMap<&str, &Section::<HttpApiClient>>>,
         };
         let gsubcommands = hashmap! {
             "delete" => delete,
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

    // *NOTE:* You can actually achieve the best of both worlds by using Arg::from() (instead of Arg::with_name())
    // and *then* setting any additional properties.
    //
    // Create an application with 5 possible arguments (2 auto generated) and 2 subcommands (1 auto generated)
    //    - A config file
    //        + Uses "-c filename" or "--config filename"
    //    - An output file
    //        + A positional argument (i.e. "$ myapp output_filename")
    //    - A debug flag
    //        + Uses "-d" or "--debug"
    //        + Allows multiple occurrences of such as "-dd" (for vary levels of debugging, as an example)
    //    - A help flag (automatically generated by clap)
    //        + Uses "-h" or "--help" (Only autogenerated if you do NOT specify your own "-h" or "--help")
    //    - A version flag (automatically generated by clap)
    //        + Uses "-V" or "--version" (Only autogenerated if you do NOT specify your own "-V" or "--version")
    //    - A subcommand "test" (subcommands behave like their own apps, with their own arguments
    //        + Used by "$ myapp test" with the following arguments
    //            > A list flag
    //                = Uses "-l" (usage is "$ myapp test -l"
    //            > A help flag (automatically generated by clap
    //                = Uses "-h" or "--help" (full usage "$ myapp test -h" or "$ myapp test --help")
    //            > A version flag (automatically generated by clap
    //                = Uses "-V" or "--version" (full usage "$ myapp test -V" or "$ myapp test --version")
    //    - A subcommand "help" (automatically generated by clap because we specified a subcommand of our own)
    //        + Used by "$ myapp help" (same functionality as "-h" or "--help")
    // Create an applicaton 
    //     - A debug flag 
    //     - A help flag
    //     - A version flag
    //     - A subcommand ghost
    //     - A subcommand twitter
    //     - A subcommand weibo
    //     - A subcommand toutiao
    //     - A subcommand wechat
    //     - A subcommand zhihu 
    //     - A subcommand othermedia
    //     - A subcommand unify
    //     - A subcommand hs
    //
    /* 
    let matches = App::new("Share")
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
        .subcommand(
            App::new("test")
                .about("does testing things")
                .arg(Arg::with_name("list").short('l').help("lists test values")),
        )
        // In the following example assume we wanted an application which
        // supported an "add" subcommand, this "add" subcommand also took
        // one positional argument of a file to add:
        .subcommand(
            App::new("ghost") // The name we call argument with
                .about("op on the ghost blog platform") // The message displayed in "myapp -h"
                // or "myapp help"
                .version("0.1") // Subcommands can have independent version
                .author("baul") // And authors
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("post") 
                         .about("post a blog")  
                         .version("0.1") 
                         .author("baul")
                         .arg(
                             Arg::with_name("blog") // And their own arguments
                                 .help("the post to post")
                                 .index(1)
                                 .required(true),
                         ),
                )
                .subcommand(
                    App::new("delete") 
                         .about("delete post")  
                         .version("0.1") 
                         .author("baul")
                         .arg(
                             Arg::with_name("post") // And their own arguments
                                 .help("the post to delete")
                                 .index(1)
                                 .required(true),
                         ),
                )
                .subcommand(
                    App::new("list") 
                         .about("list posts")  
                         .version("0.1") 
                         .author("baul")
                         .arg(
                             Arg::with_name("posts") // And their own arguments
                                 .help("list post")
                                 .index(1)
                                 .required(true),
                         ),
                )
        )
        .get_matches();
                                 */ 

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
       println!("section_name: {}", section_name);
      // if (section_name == &"test") {continue;}
       let mut subcommand = App::new(section_name.to_string()).about(section.description);
       for arg in &section.args {
           subcommand = subcommand.arg(arg);
       }
       match section.function {
           Some(f) => println!("have action,level end:{}",section_name), 
           None => { 
               println!("no action,have subcommands:{}",section_name);
               // docmds loop subcommands
               // println!("subcommands :{:#?}", section.subcommands.as_ref().unwrap());
               for (section_name, section) in section.subcommands.as_ref().unwrap().iter() {
                   println!("ghost section_name{}", section_name);
                   let mut ssubcommand = App::new(section_name.to_owned()).about(section.description);
                   for arg in &section.args {
                       ssubcommand = ssubcommand.arg(arg);
                   }
                   subcommand = subcommand.subcommand(ssubcommand);
            
               }

           },
       }
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
                ("post", Some(post_matches)) => {
                    println!("to post posts/pages { }", post_matches.value_of("blog").unwrap());
                    // call lib ghost
                    // why need await ,only no use some await
                    let resp = ghost::post().await?;
                    println!("list post {:#?}", resp); //mean
                }

                ("delete", Some(delete_matches)) => {
                    // Now we have a reference to delete's matches
                    println!("this post url {} will be deleted", delete_matches.value_of("post").unwrap());
                    // call lib ghost
                   // let resp = ghost::delete().await();
                    // let resp: std::collections::HashMap<std::string::String, std::string::String> = ghost::delete().await?;
                    let resp = ghost::delete().await?;
                    //let resp: u32 = ghost::delete().await?;
                    println!("{:#?}", resp); //mean

                  // no need all below becaseu we have use the ? and last OK(()) to return result.
                  //  let resp = match resp {
                  //      Ok(res) => {
                  //          println!("resp in match: {}", res);
                  //      },
                  //      Err(error) => {
                  //          panic!("Problem deleting post : {:?}", error)
                  //      },
                  //  };
                    //Ok(()) // why cannot add this 

                }
                ("list", Some(list_matches)) => {
                    println!("to list posts/pages { }", list_matches.value_of("posts").unwrap());
                    let resp = ghost::list().await?;
                    println!("list post {:#?}", resp); //mean
                }
                ("", None) => println!("No ghost subcommand was used"), // If no subcommand was usd it'll match the tuple ("", None)
                _ => unreachable!(),
            }
        }
        ("", None) => println!("No main subcommand was used"), // If no subcommand was usd it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    
//  
//      // An alternative to checking the name is matching on known names. Again notice that only the
//      // direct children are matched here.
//      match matches.subcommand_name() {
//          Some("clone") => println!("'git clone' was used"),
//          Some("push") => println!("'git push' was used"),
//          Some("add") => println!("'git add' was used"),
//          None => println!("No subcommand was used"),
//          _ => unreachable!(), // Assuming you've listed all direct children above, this is unreachable
//      }
//  
    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("debug") {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        3 | _ => println!("Don't be crazy"),
    }

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


