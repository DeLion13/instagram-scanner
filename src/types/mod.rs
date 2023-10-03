#![allow(dead_code)]
use std::time::SystemTime;

/// Defines the privacy type of the account
enum AccountPrivacy {
    Private,
    Public,
}

/// The main account to be scanned
pub struct Account {
    id: String,

    /// Nickname of the account; main identifier
    nickname: String,

    /// Link to the instagram account
    link: String,

    /// Privacy settings
    status: AccountPrivacy,

    /// Last time account was scanned
    last_scan: SystemTime,

    /// Differences recorded previously
    cached_diff: Vec<Difference>,

    /// This account's followers
    followers_list: AccountsList,

    /// Accounts this one follows
    following_list: AccountsList,
}

/// List of minimalistic accounts that might be used to
/// represent either followers or accounts one's following
pub struct AccountsList {
    id: String,
    list: Vec<MinimalisticAccount>,
}

/// Minimised representation of account
pub struct MinimalisticAccount {
    /// Nickname of the account; main identifier
    nickname: String,

    /// Link to the instagram account
    link: String,

    /// Privacy settings
    status: AccountPrivacy,
}

/// Difference between last and current scan
pub struct Difference {
    id: String,

    /// Accounts that were added to followers list
    new_followers: Vec<MinimalisticAccount>,

    /// Accounts that were removed from followers list
    new_unfollows: Vec<MinimalisticAccount>,

    /// Previous time this account was scanned
    previous_scan_time: SystemTime,

    /// Account scanning time
    current_scan_time: SystemTime,
}
