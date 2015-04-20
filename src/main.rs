#![feature(core)]
#![feature(std_misc)]
#![feature(rustc_private)]
#![feature(collections)]
#![feature(exit_status)]
extern crate notify;
extern crate getopts;
use std::env;
use std::time::duration::Duration;
use getopts::{optopt,optflag,getopts};
use std::string::String;


fn main() {
    let args: Vec<String> = env::args().collect();
    let ref program = args[0];

    let opts = [
        getopts::optopt("u", "urgency", "Specifies urgency level (low, normal, critical)", "LEVEL"),
        getopts::optopt("t", "expire-time", "Specifies the timeout at which to expire the notification", "TIME"),
        getopts::optopt("i", "icon", "Specifies the icon filename or stock icon to display.", "ICON"),
        getopts::optopt("c", "category", "Specifies the notification category", "TYPE"),
        getopts::optflag("h", "help", "Print this help")
    ];

    let blank = String::from_str("");

    let matches = match getopts::getopts(args.tail(), &opts){
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            env::set_exit_status(1);
            return;
            // The exit code is 0 (success) by default.
            // Any exit code other than 0 indicates failure.
        }
    };

    if matches.opt_present("h") || (matches.free.len() < 1) {
        let brief = format!("Usage: {} [options] <summary> [description]", program);
        println!("{}", getopts::usage(brief.as_slice(), &opts));
        return;
    }

    let summary = matches.free[0].clone();
    let description = matches.free[0].clone();
    let icon = matches.opt_str("i").unwrap_or(blank.clone());

    notify::init(args[0].as_slice());
    let notification = notify::Notification::new(summary.as_slice(), description.as_slice(), icon.as_slice());

    match matches.opt_str("c") {
        Some(category) => notification.set_category(category.as_slice()),
        None => ()
    }
    match matches.opt_str("t") {
        Some(expires) => {
            let expires: i64 = expires.parse::<i64>().unwrap();
            let expires = Duration::milliseconds(expires);
            notification.set_timeout(expires);
        },
        None => ()
    }

    notification.show();
    notify::uninit();
}
