// use clap::{App, Clap};
// 
// #[derive(Clap)]
// #[clap(version = "v1.0-beta")]
// struct Opts;
// fn main() {
//     Opts::parse();
// }



// extern crate greetings;
// 
// fn main() {
//     greetings::hello();
// }

//extern crate calp;
extern crate ghost;
extern crate clap;
extern crate tokio;
// use clap::{App, Clap, Arg, AppSettings};
use clap::{App, Arg, AppSettings};

#[tokio::main]
//async fn main() -> Result<(), std::io::Error> {
// Box any error
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        )
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(o) = matches.value_of("output") {
        println!("Value for output: {}", o);
    }
    if matches.is_present("ghost") {
        println!("'sharecli ghost' was run.");
    }

    // You can see which subcommand was used
    if let Some(subcommand) = matches.subcommand_name() {
        println!("'sharecli {}' was used", subcommand);

        // It's important to note, this *only* check's git's DIRECT children, **NOT** it's
        // grandchildren, great grandchildren, etc.
        //
        // i.e. if the command `git push remove --stuff foo` was run, the above will only print out,
        // `git push` was used. We'd need to get push's matches to see futher into the tree
    }

    // You could get the independent subcommand  matches, including children , although this is less common
    if let Some(clone_matches) = matches.subcommand_matches("clone") {
        // Now we have a reference to clone's matches
        println!("Cloning repo: {}", clone_matches.value_of("repo").unwrap());
    }
 //       
 //           // The most common way to handle subcommands is via a combined approach using
 //       // `ArgMatches::subcommand` which returns a tuple of both the name and matches
 //       match matches.subcommand() {
 //           ("clone", Some(clone_matches)) => {
 //               // Now we have a reference to clone's matches
 //               println!("Cloning {}", clone_matches.value_of("repo").unwrap());
 //           }
 //           ("push", Some(push_matches)) => {
 //               // Now we have a reference to push's matches
 //               match push_matches.subcommand() {
 //                   ("remote", Some(remote_matches)) => {
 //                       // Now we have a reference to remote's matches
 //                       println!("Pushing to {}", remote_matches.value_of("repo").unwrap());
 //                   }
 //                   ("local", Some(_)) => {
 //                       println!("'git push local' was used");
 //                   }
 //                   _ => unreachable!(),
 //               }
 //           }
 //           ("add", Some(add_matches)) => {
 //               // Now we have a reference to add's matches
 //               println!(
 //                   "Adding {}",
 //                   add_matches
 //                       .values_of("stuff")
 //                       .unwrap()
 //                       .collect::<Vec<_>>()
 //                       .join(", ")
 //               );
 //           }
 //           ("", None) => println!("No subcommand was used"), // If no subcommand was usd it'll match the tuple ("", None)
 //           _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
 //       }
 //       // my ghost
    match matches.subcommand() {
        ("ghost", Some(ghost_matches)) => {
            // Now we have a reference to ghost's matches
            match ghost_matches.subcommand() {
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
                _ => unreachable!(),
            }
        }
        ("", None) => println!("No subcommand was used"), // If no subcommand was usd it'll match the tuple ("", None)
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

