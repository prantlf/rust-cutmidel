use std::env;
use std::process;
use std::str::FromStr;

const VERSION: &str = "1.0.0";
const HINT: &str = "use -h to get help";

fn usage() {
    println!(
        "cutmidel {VERSION}
  Shortens a text by trimming it by an ellipsis in the middle.

Usage: cutmidel [options] <text> <leading> <trailing> [ellipsis]

  Specify the text and the maximum count of leading and trailing
  characters. The overall maximum length will be their sum plus
  the length of an ellipsis (3 dots by default). Zero for either
  leading or trailing count means no leading or trailing parts.

Options:
  -V|--version  prints the version of the executable and exits
  -h|--help     prints th usage information and exits

Examples:
  $ cutmidel \"~/Sources/private/cutmidel\" 5 10
  ~/Sou...e/cutmidel
  $ cutmidel ~/Sources/private/cutmidel 0 12 ..
  ..ate/cutmidel
"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    // print usage information if no arguments were provided
    if argc == 1 {
        usage();
        process::exit(1);
    }
    // check if printing the version number or usage information was requested
    if argc == 2 {
        let arg = &args[1];
        if arg == "-V" || arg == "--version" {
            println!("{VERSION}");
            process::exit(0);
          }
        if arg == "-h" || arg == "--help" {
            usage();
            process::exit(0);
        }
        // unexpected one argument or fail if unexpected arguments were not provided
        println!("invalid argument: {arg} ({HINT})");
        process::exit(1);
    }
    // check if exactly three arguments are available
    if argc == 3 {
        println!("too few arguments ({HINT})");
        process::exit(1);
    }
    if argc > 5 {
        println!("too many arguments ({HINT})");
        process::exit(1);
    }

    // make sure that leading and trailing character count are numeric
    let lead_arg = &args[2];
    let lead = usize::from_str(lead_arg).unwrap_or_else(|_| {
      println!("invalid leading character count: \"{lead_arg}\" ({HINT})");
      process::exit(1);
    });
    let trail_arg = &args[3];
    let trail = usize::from_str(trail_arg).unwrap_or_else(|_| {
      println!("invalid trailing character count: \"{trail_arg}\" ({HINT})");
      process::exit(1);
    });
    // ellipsis cannot be put to the middle unless the middle is specified
    if lead == 0 && trail == 0 {
      println!("both leading and trailing counts cannot be zero ({HINT})");
      process::exit(1);
    }
    // check if a custom ellipsis was specified
    let ellipsis: &str;
    if argc == 5 {
      ellipsis = &args[4];
    } else {
      ellipsis = "...";
    }
    let elliplen = ellipsis.len();

	// get the text to trim with its length; it will be trimmed in-place
	let txt = &args[1];
	let txtlen = txt.len();

	// if the input text is shorter than the ledting and trailing character
	// count plus the ellipsis length, leave it intact
  let mut out = String::with_capacity(txtlen);
	if txtlen > lead + trail + elliplen {
		if lead == 0 {
			// if only the trailing part should be displayed, put the ellipsis
			// to the beginning and move the trailing part behind it
      out.push_str(ellipsis);
      out.push_str(&txt[txtlen - trail..]);
		} else if trail == 0 {
			// if only the leading part should be displayed, put the ellipsis
			// right behind it
      out.push_str(&txt[..lead]);
      out.push_str(ellipsis);
		} else {
			// if both leading and trailing parts should be displayed, put the
			// ellipsis behind the leading part and move the trailing part behind it
      out.push_str(&txt[..lead]);
      out.push_str(ellipsis);
      out.push_str(&txt[txtlen - trail..]);
		}
	} else {
    out.push_str(txt);
  }

  print!("{out}")
}
