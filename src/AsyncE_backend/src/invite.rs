use crate::{
    globals::{GROUPS, GROUP_INVITES, USERS},
    user, websocket,
};

#[ic_cdk::update]
pub fn invite_user(group_id: u128, username: String) -> Result<(), String> {
    user::assert_user_logged_in()?;

    let selfname = user::get_selfname_force()?;

    GROUPS.with_borrow(|groups| {
        let group = groups
            .get(&group_id)
            .ok_or(String::from("Cannot find group with this ID!"))?;

        if group.owner != selfname && !group.users.contains(&selfname) {
            return Err(String::from("This user is not in this group!"));
        }

        if group.users.contains(&username) {
            return Err(String::from("Chosen user is already in this group!"));
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
pub fn get_self_group_invites() -> Result<Vec<u128>, String> {
    user::assert_user_logged_in()?;

    let selfname = user::get_selfname_force()?;

    Ok(GROUP_INVITES.with_borrow(|group_invites| {
        group_invites
            .get(&selfname)
            .cloned()
            .unwrap_or_default()
            .iter()
            .cloned()
            .collect::<Vec<_>>()
    }))
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

                group.users.push(selfname);
            }

            Ok(())
        })
    })
}
