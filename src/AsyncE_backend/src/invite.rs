use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    globals::{GROUPS, GROUP_INVITES, USERS},
    user, websocket,
};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GroupInviteResponse {
    pub group_id: u128,
    pub group_name: String,
}

#[ic_cdk::update]
pub fn invite_user(group_id: u128, username: String) {
    let selfname = user::get_selfname().unwrap();

    GROUPS.with_borrow(|groups| {
        let group = groups
            .get(&group_id)
            .expect("Cannot find group with this ID!");
        if group.owner != selfname && !group.users.contains(&selfname) {
            panic!("This user is not in this group!")
        }

        if group.users.contains(&username) {
            panic!("Chosen user is already in this group!")
        }

        GROUP_INVITES.with_borrow_mut(|group_invites| {
            let group_invites = group_invites.entry(username.clone()).or_default();
            if group_invites.contains(&group_id) {
                panic!("Chosen user is already invited to this group!")
            }

            USERS.with_borrow(|users| {
                let principal = users
                    .iter()
                    .find(|user| user.1.username.eq_ignore_ascii_case(&username))
                    .map(|x| x.0.clone())
                    .expect("Cannot find user with this username!");
                group_invites.insert(group_id);

                websocket::send_group_invited_notif(principal, group.id, &group.name);
            })
        })
    })
}

#[ic_cdk::query]
pub fn get_self_group_invites() -> Vec<GroupInviteResponse> {
    let selfname = user::get_selfname().unwrap();

    GROUP_INVITES.with_borrow(|group_invites| {
        group_invites
            .get(&selfname)
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|x| GroupInviteResponse {
                group_id: *x,
                group_name: GROUPS.with_borrow(|groups| groups.get(x).unwrap().name.clone()),
            })
            .collect::<Vec<_>>()
    })
}

#[ic_cdk::update]
pub fn update_group_invite(group_id: u128, approved: bool) {
    user::assert_user_logged_in();

    let selfname = user::get_selfname().unwrap();

    GROUP_INVITES.with_borrow_mut(|group_invites| {
        let group_invites = match group_invites.get_mut(&selfname) {
            Some(group_invites) => group_invites,
            None => panic!("Cannot find invite data on this group"),
        };

        if !group_invites.contains(&group_id) {
            panic!("Cannot find invite data on this group")
        }

        GROUPS.with_borrow_mut(|groups| {
            group_invites.remove(&group_id);

            if approved {
                let group = groups
                    .get_mut(&group_id)
                    .expect("Cannot find group with this ID!");
                group.users.push(selfname);
            }
        });
    });
}
