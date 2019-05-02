mod authorize;
mod authenticate;
mod timeline;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "twt", about = "The Twitter API Command Line Interface for serious developers")]
pub enum Cli {
    #[structopt(name = "authorize", about = "Authorize twt-cli to access your account")]
    Authorize,
    #[structopt(name = "timeline", about = "Gets timeline of active or provided user")]
    Timeline {
        #[structopt(short = "u", long = "user")]
        user: Option<String>,
        count: Option<i8>
    },
    #[structopt(name = "followers", about = "Gets followers of active or provided user")]
    Followers {
        #[structopt(short = "u", long = "user")]
        user: Option<String>
    },
}

fn main() -> std::io::Result<()> {
    let command = Cli::from_args();

    match &command {
        Cli::Authorize {} => authorize::authorize(command),
        Cli::Timeline { user, count:_ } => timeline::get_timeline(user),
        Cli::Followers { user:_ } => get_followers(command),
    }

    Ok(())
}

fn get_followers(args: Cli) -> () {
    println!("Followers triggered = {:?}", &args);
}