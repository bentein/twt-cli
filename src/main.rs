mod authorize;
mod authenticate;
mod timeline;
mod tweet;

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
        #[structopt(short = "c", long = "count")]
        count: Option<String>,
        #[structopt(long = "max")]
        max: Option<String>,
    },
    #[structopt(name = "followers", about = "Gets followers of active or provided user")]
    Followers {
        #[structopt(short = "u", long = "user")]
        user: Option<String>,
    },
    #[structopt(name = "tweet", about = "Posts a tweet as active user")]
    Tweet {
        #[structopt(required_unless = "delete", required_unless = "show", conflicts_with = "delete", conflicts_with = "show")]
        status: Option<String>,
        #[structopt(short = "d", long = "delete")]
        delete: Option<String>,
        #[structopt(long = "id", conflicts_with = "delete")]
        show: Option<String>,
    }
}

fn main() -> std::io::Result<()> {
    let command = Cli::from_args();

    match &command {
        Cli::Authorize {} => authorize::authorize(command),
        Cli::Timeline { user, count , max} => { timeline::get_timeline(user, count, max)?; },
        Cli::Followers { user:_ } => get_followers(command),
        Cli::Tweet { status, delete, show } => { tweet::tweet(status, delete, show)?; },
    }

    Ok(())
}

fn get_followers(args: Cli) -> () {
    println!("Followers triggered = {:?}", &args);
}