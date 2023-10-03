use std::time::SystemTime;

enum AccountPrivacy {
    Private,
    Public
}

pub struct Account {
    id: String,
    nickname: String,
    link: String,
    status: AccountPrivacy,
    last_scan: SystemTime,
    cached_diff: Vec<Difference>,

    subscribers_list: AccountsList,
    subscribes_list: AccountsList,
}

pub struct AccountsList {
    id: String,
    list: Vec<MinimalisticAccount>
}

pub struct MinimalisticAccount {
    nickname: String,
    link: String,
    status: AccountPrivacy,
}

pub struct Difference {
    id: String,
    new_subscribers: Vec<MinimalisticAccount>,
    new_unsubscribes: Vec<MinimalisticAccount>,
    previous_scan_time: SystemTime,
    current_scan_time: SystemTime,
}
