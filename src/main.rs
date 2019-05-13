mod authorize;
mod authenticate;
mod credentials;
mod followers;
mod friendships;
mod timeline;
mod tweet;

mod util;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "twt", about = "The Twitter API Command Line Interface for serious developers")]
pub enum Cli {
    #[structopt(name = "authorize", about = "Authorize twt-cli to access your account")]
    Authorize {},
    #[structopt(name = "credentials", about = "Modify stored credentials used by twt-cli")]
    Credentials {
        #[structopt(subcommand)]
        credentials_type: CredentialsType,
    },
    #[structopt(name = "followers", about = "Gets followers of active or provided user")]
    Followers {
        #[structopt(short = "u", long = "user")]
        user: Option<String>,
    },
    #[structopt(name = "follow", about = "Follows provided user")]
    Follow {
        screen_name: String,
        #[structopt(short = "n", long = "notifications")]
        notifications: bool,
    },
    #[structopt(name = "unfollow", about = "Unfollows provided user")]
    Unfollow {
        screen_name: String,
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
pub enum CredentialsType {
    #[structopt(name = "app")]
    App {
        #[structopt(subcommand)]
        action: AppMod,
    },
    #[structopt(name = "user")]
    User {
        #[structopt(subcommand)]
        action: UserMod,
    },
}

#[derive(StructOpt, Debug)]
pub enum AppMod {
    #[structopt(name = "add")]
    Add {
        token: String,
        secret: String,
    },
}

#[derive(StructOpt, Debug)]
pub enum UserMod {
    #[structopt(name = "add")]
    Add {
        name: String,
        token: String,
        secret: String,
    },
    #[structopt(name = "activate")]
    Activate {
        name: String,
    },
    #[structopt(name = "delete")]
    Delete {
        name: String,
    },
    #[structopt(name = "set")]
    Set {
        name: String,
        token: String,
        secret: String,
    },
}

fn main() -> std::io::Result<()> {
    let command = Cli::from_args();

    let res: String = match &command {
        Cli::Authorize { } => authorize::authorize()?,
        Cli::Credentials { credentials_type } => do_credentials(credentials_type)?,
        Cli::Follow { screen_name, notifications } => friendships::create_friendship(screen_name, notifications)?,
        Cli::Unfollow { screen_name } => friendships::destroy_friendship(screen_name)?,
        Cli::Followers { user:_ } => String::new(),
        Cli::Timeline { user, count , max_id} => timeline::get_timeline(user, count, max_id)?,
        Cli::Tweet { status, delete, show } => tweet::tweet(status, delete, show)?,
    };

    println!("{}", res);

    Ok(())
}

fn do_credentials(credentials_type: &CredentialsType) -> std::io::Result<String> {

    match credentials_type {
        CredentialsType::App { action } => {

            match action {
                AppMod::Add { token, secret} => credentials::set_application_credentials(token, secret),
            }
        },
        CredentialsType::User { action} => {

            match action {

                UserMod::Add { name, token, secret } => credentials::add_new_user_credentials(name, token, secret),
                UserMod::Activate { name } => credentials::set_active_user(name),
                UserMod::Delete { name } => credentials::delete_user_credentials(name),
                UserMod::Set { name, token, secret } => Ok("Not yet implemented".to_string()),

            }
        }
    }
}

fn get_followers(args: Cli) -> () {
    println!("Followers triggered = {:?}", &args);
}