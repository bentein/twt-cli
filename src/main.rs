mod authorize;
mod authenticate;
mod timeline;
mod tweet;
mod followers;
mod credentials;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "twt", about = "The Twitter API Command Line Interface for serious developers")]
pub enum Cli {
    #[structopt(name = "authorize", about = "Authorize twt-cli to access your account")]
    Authorize {
        #[structopt(subcommand)]
        auth_type: AuthType,
    },
    #[structopt(name = "timeline", about = "Gets timeline of active or provided user")]
    Timeline {
        #[structopt(short = "u", long = "user")]
        user: Option<String>,
        #[structopt(short = "c", long = "count")]
        count: Option<String>,
        #[structopt(long = "max")]
        max_id: Option<String>,
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

#[derive(StructOpt, Debug)]
pub enum AuthType {
    #[structopt(name = "user")]
    User {
        #[structopt(short = "d", long = "delete", conflicts_with = "activate")]
        delete: bool,
        #[structopt(short = "a", long = "active")]
        active: bool,
        name: String,
        #[structopt(required_unless = "delete", required_unless = "active")]
        token: Option<String>,
        #[structopt(required_unless = "delete", required_unless = "active")]
        secret: Option<String>,
    },
    #[structopt(name = "app")]
    App {
        token: String,
        secret: String,
    }
}


fn main() -> std::io::Result<()> {
    let command = Cli::from_args();

    let res: String = match &command {
        Cli::Authorize { auth_type } => do_authorize(auth_type)?,
        Cli::Timeline { user, count , max_id} => timeline::get_timeline(user, count, max_id)?,
        Cli::Followers { user:_ } => String::new(),
        Cli::Tweet { status, delete, show } => tweet::tweet(status, delete, show)?,
    };

    println!("{}", res);

    Ok(())
}

fn do_authorize(auth_type: &AuthType) -> std::io::Result<String> {

    match auth_type {
        AuthType::User { delete, active, name, token, secret} =>
            authorize::authorize_user(delete, active, name, token, secret),
        AuthType::App { token, secret } => authorize::authorize_app(token, secret),
    }

}

fn get_followers(args: Cli) -> () {
    println!("Followers triggered = {:?}", &args);
}