use candid::CandidType;
use serde::{Deserialize, Serialize};

use crate::{
    globals::{GROUPS, GROUP_INVITES, USERS},
    group::{GroupMember, GroupMemberRole},
    user, websocket,
};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct GroupInviteResponse {
    pub group_id: u128,
    pub group_name: String,
}

impl GroupInviteResponse {
    fn new(group_id: u128) -> Result<Self, String> {
        GROUPS.with_borrow(|groups| {
            let group_name = groups
                .get(&group_id)
                .ok_or(String::from("Cannot find this group with the provided ID!"))?
                .name
                .clone();

            Ok(Self {
                group_id,
                group_name,
            })
        })
    }
}

#[ic_cdk::update]
pub fn invite_user(group_id: u128, username: String) -> Result<(), String> {
    user::assert_user_logged_in()?;

    let selfuser =
        user::get_selfuser()?.ok_or(String::from("This user does not have a username!"))?;

    GROUPS.with_borrow(|groups| {
        let group = groups
            .get(&group_id)
            .ok_or(String::from("Cannot find group with this ID!"))?;

        if !group.is_member(&selfuser.username) {
            return Err(String::from("This user is not in this group!"));
        }

        if group.is_member(&username) {
            return Err(String::from("Chosen user is already in this group!"));
        }

        if selfuser.subscription.is_none() && group.members.len() >= 10 {
            return Err(String::from(
                "User must be subscribed to invite more users!",
            ));
        }

        GROUP_INVITES.with_borrow_mut(|group_invites| {
            let group_invites = group_invites.entry(username.clone()).or_default();
            if group_invites.contains(&group_id) {
                return Err(String::from(
                    "Chosen user is already invited to this group!",
                ));
            }

            USERS.with_borrow(|users| {
                let principal = users
                    .iter()
                    .find(|user| user.1.username.eq_ignore_ascii_case(&username))
                    .map(|x| *x.0)
                    .ok_or(String::from("Cannot find user with this username!"))?;

                group_invites.insert(group_id);
                websocket::send_group_invited_notif(principal, group.id, &group.name);

                Ok(())
            })
        })
    })
}

#[ic_cdk::query]
pub fn get_self_group_invites() -> Result<Vec<GroupInviteResponse>, String> {
    user::assert_user_logged_in()?;

    let selfname = user::get_selfname_force()?;

    GROUP_INVITES.with_borrow(|group_invites| {
        group_invites
            .get(&selfname)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(GroupInviteResponse::new)
            .collect::<_>()
    })
}

#[ic_cdk::update]
pub fn update_group_invite(group_id: u128, approved: bool) -> Result<(), String> {
    user::assert_user_logged_in()?;

    let selfname = user::get_selfname_force()?;

    GROUP_INVITES.with_borrow_mut(|group_invites| {
        let group_invites = group_invites
            .get_mut(&selfname)
            .ok_or(String::from("Cannot find invite data on this group"))?;

        if !group_invites.contains(&group_id) {
            return Err(String::from("Cannot find invite data on this group"));
        }

        GROUPS.with_borrow_mut(|groups| {
            group_invites.remove(&group_id);

            if approved {
                let group = groups
                    .get_mut(&group_id)
                    .ok_or(String::from("Cannot find group with this ID!"))?;

                group
                    .members
                    .push(GroupMember::new(selfname, GroupMemberRole::Member));
            }

            Ok(())
        })
    })
}
