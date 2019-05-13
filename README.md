# twt - The twitter API command line interface

twt is the Twitter API CLI that covers all (some of) your Twitter API scripting needs. Development is ongoing. All APIs  are subject to breaking changes.

## Getting Started

To use `twt`, you must first install Rust on your computer. 

With rust installed, clone this project and run `cargo build --release`. You will find the newly built twt binary in the `target` folder.

To be able to send requests to the Twitter API, you need to create a Twitter developer account, and a Twitter API application, at https://developer.twitter.com. Once you have created an account and an application, you need to add your credentials to twt.

Add your application credentials by running `twt credentials app add <application-key> <application-secret>`.

To use most of twt's features, you need to add user credentials as well. You can authorize the application to use an account by running `twt authorize`. 

These credentials can also be manually added by running `twt credentials user add <username> <oauth-key> <oauth-secret>`. Remember that you must already have authorized the application to access your account for these credentials to work.

Several users can be added to the application configuration. You can change the active user by running `twt credentials user activate <username>`.

You can remove a set of user credentials by running `twt credentials user delete <username>`.

## Usage

Currently, the following Twitter APIs are implemented:

#####Timeline

* statuses/home_timeline
* statuses/user_timeline

#####Tweets

* statuses/update
* statuses/destroy/:id
* statuses/show/:id

#####Friendships

* friendships/create
* friendships/destroy

To see all subcommands, run `twt --help`