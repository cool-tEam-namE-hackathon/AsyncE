use candid::CandidType;
use serde::Deserialize;

use crate::{
    chunk,
    globals::{MEETINGS, VIDEO_UPLOADS},
    group, http,
    primary_key::{self, PrimaryKeyType},
    user, websocket,
};

#[derive(Copy, Clone, Debug, Default, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MeetingProcessType {
    #[default]
    None,
    Concat,
    Subtitle
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Meeting {
    pub id: u128,
    pub thumbnail_data: Vec<u8>,
    pub full_video_data: Vec<u8>,

    pub title: String,
    pub created_by: String,

    pub frames: Vec<VideoFrame>,
    pub created_time_unix: u128,

    pub process_type: MeetingProcessType
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct MeetingHeader {
    pub id: u128,
    pub title: String,
    pub created_by: String,
    pub frames_count: u128,
    pub created_time_unix: u128,
    pub process_type: MeetingProcessType
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct VideoFrame {
    pub data: Vec<u8>,
    pub title: String,
    pub created_by: String,
    pub thumbnail_data: Vec<u8>,
    pub created_time_unix: u128,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct VideoFrameHeader {
    pub title: String,
    pub created_by: String,
    pub created_time_unix: u128,
}

impl From<&VideoFrame> for VideoFrameHeader {
    fn from(value: &VideoFrame) -> Self {
        Self {
            title: value.title.clone(),
            created_by: value.created_by.clone(),
            created_time_unix: value.created_time_unix,
        }
    }
}

impl VideoFrame {
    fn new(username: String, title: String) -> Self {
        Self {
            data: Vec::new(),
            title,
            created_by: username,
            thumbnail_data: Vec::new(),
            created_time_unix: ic_cdk::api::time() as u128,
        }
    }
}

impl Meeting {
    pub fn new(username: String, title: String) -> Self {
        Self {
            id: primary_key::get_primary_key(PrimaryKeyType::Video),
            full_video_data: Vec::new(),
            thumbnail_data: Vec::new(),
            frames: Vec::new(),
            title,
            created_by: username,
            created_time_unix: ic_cdk::api::time() as u128,
            process_type: MeetingProcessType::None
        }
    }
}

impl From<&Meeting> for MeetingHeader {
    fn from(value: &Meeting) -> Self {
        Self {
            id: value.id,
            title: value.title.clone(),
            created_by: value.created_by.clone(),
            frames_count: value.frames.len() as u128,
            created_time_unix: value.created_time_unix,
            process_type: value.process_type
        }
    }
}

fn assert_check_group(group_id: u128) -> Result<(), String> {
    let group =
        group::get_group(group_id)?.ok_or(String::from("Group with this ID is not found"))?;

    let name = user::get_selfname_force()?;

    if group.owner != name && !group.members.iter().any(|x| x.username.eq_ignore_ascii_case(&name)) {
        return Err(String::from("Current user does not belong to this group!"));
    }

    Ok(())
}

#[ic_cdk::query]
pub fn get_meetings(group_id: u128) -> Result<Vec<MeetingHeader>, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    Ok(MEETINGS
        .lock()
        .unwrap()
        .get(&group_id)
        .cloned()
        .unwrap_or_default()
        .values()
        .map(MeetingHeader::from)
        .collect::<Vec<_>>())
    
}

#[ic_cdk::query]
pub fn get_meeting_detail(group_id: u128, meeting_id: u128) -> Result<MeetingHeader, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    MEETINGS
        .lock()
        .unwrap()
        .get(&group_id)
        .ok_or(String::from("No meetings found on this group!"))?
        .get(&meeting_id)
        .ok_or(String::from("No meeting found on this meeting ID!"))
        .map(MeetingHeader::from)
    
}

#[ic_cdk::update]
pub fn create_meeting(group_id: u128, title: String) -> Result<u128, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    let selfname = user::get_selfname_force()?;
    let meeting = Meeting::new(selfname.clone(), title.clone());
    let meeting_id = meeting.id;

    MEETINGS
        .lock()
        .unwrap()
        .entry(group_id)
        .or_default()
        .insert(meeting_id, meeting);

    Ok(meeting_id)
}

#[ic_cdk::update]
pub fn upload_video(
    group_id: u128,
    meeting_id: u128,
    data: Vec<u8>,
    finish: bool,
    title: String,
    video_upload_uuid: String,
    chunk_index: u128,
    total_data_length: u128,
    with_subtitles: bool
) -> Result<(), String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    let selfname = user::get_selfname_force()?;

    let mut meetings = MEETINGS.lock().unwrap();
    let meetings = meetings
        .get_mut(&group_id)
        .ok_or(String::from("No meetings found on this group!"))?;

    let meeting = meetings
        .get_mut(&meeting_id)
        .ok_or(String::from("No meeting found on this video ID!"))?;

    if meeting.process_type != MeetingProcessType::None {
        return Err(String::from("Video is still on procesing... Please try again later.."))
    }

    ic_cdk::println!("Cool");

    VIDEO_UPLOADS.with_borrow_mut(|video_uploads| {
        let video_upload = video_uploads
            .entry(video_upload_uuid.clone())
            .or_default();

        if video_upload.capacity() != total_data_length as usize {
            video_upload.reserve_exact(total_data_length as usize);
        }

        let offset = chunk_index as usize * chunk::MB;
        video_upload.splice(offset..offset, data);

        if finish {
            let data = video_uploads.remove(&video_upload_uuid).ok_or(String::from(
                "Cannot find existing upload process with given UUID (This should never happen though)",
            ))?;

            if meeting.full_video_data.is_empty() {                
                meeting.full_video_data = data.clone();
            } else if !with_subtitles {
                meeting.process_type = MeetingProcessType::Concat;
                http::send_concat_video_request(group_id, meeting_id, meeting.full_video_data.clone(), data.clone())
            }

            let mut video_frame = VideoFrame::new(selfname.clone(), title);
            video_frame.data = data.clone();
            meeting.frames.push(video_frame);

            if with_subtitles {
                meeting.process_type = MeetingProcessType::Subtitle;
                send_process_subtitles_request(group_id, meeting_id, meeting.frames.len() - 1, data.clone());
            }
            
            get_thumbnail_from_video_data(group_id, meeting_id, meeting.frames.len() - 1, data.clone());
            websocket::broadcast_new_video_part(group_id, meeting_id, selfname);
        }

        Ok(())
    })
}

fn send_process_subtitles_request(group_id: u128, meeting_id: u128, index: usize, data: Vec<u8>) {
    ic_cdk::spawn(async move {
        if let Err(err) = http::send_process_subtitles_request(group_id, meeting_id, index, data).await {
            ic_cdk::eprintln!("Failed to send process subtitles request: {}", err)
        }
    });
}

fn get_thumbnail_from_video_data(group_id: u128, meeting_id: u128, frame_index: usize, data: Vec<u8>) {
    ic_cdk::spawn(async move {
        let thumbnail_data = match http::send_thumbnail_request(data).await {
            Ok(thumbnail_data) => thumbnail_data,
            Err(err) => {
                ic_cdk::eprintln!("Error while sending thumbnail request: {}", err);
                return;
            }
        };

        let mut meetings = MEETINGS.lock().unwrap();
        let meetings = meetings
            .get_mut(&group_id)
            .ok_or(String::from("No meetings found on this group!"))
            .unwrap();

        let meeting = meetings
            .get_mut(&meeting_id)
            .ok_or(String::from("No meeting found on this video ID!"))
            .unwrap();
        if meeting.thumbnail_data.is_empty() {
            meeting.thumbnail_data = thumbnail_data.clone();
        }

        meeting
            .frames
            .get_mut(frame_index)
            .ok_or(String::from("No frame found on this meeting index!"))
            .unwrap()
            .thumbnail_data = thumbnail_data;
    })
}

#[ic_cdk::query]
pub fn get_video_meeting_size(group_id: u128, meeting_id: u128) -> Result<u128, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    let meetings = MEETINGS.lock().unwrap();
    let meetings = meetings
        .get(&group_id)
        .ok_or(String::from("No meetings found on this group!"))?;

    let meeting = meetings
        .get(&meeting_id)
        .ok_or(String::from("No meeting found on this meeting ID!"))?;

    Ok(meeting.full_video_data.len() as u128)
}

#[ic_cdk::query]
pub fn get_video_meeting_chunk_blob(
    group_id: u128,
    meeting_id: u128,
    index: u128,
) -> Result<Vec<u8>, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    let meetings = MEETINGS.lock().unwrap();
    let meetings = meetings
        .get(&group_id)
        .ok_or(String::from("No meetings found on this group!"))?;

    let meeting = meetings
        .get(&meeting_id)
        .ok_or(String::from("No meeting found on this meeting ID!"))?;

    Ok(meeting
        .full_video_data
        .iter()
        .skip(index as usize * chunk::MB)
        .take(chunk::MB)
        .cloned()
        .collect())
}

#[ic_cdk::query]
pub fn get_video_frame_size(
    group_id: u128,
    meeting_id: u128,
    frame_index: u128,
) -> Result<u128, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    let meetings = MEETINGS.lock().unwrap();
    let meetings = meetings
        .get(&group_id)
        .ok_or(String::from("No meetings found on this group!"))?;

    let meeting = meetings
        .get(&meeting_id)
        .ok_or(String::from("No meeting found on this meeting ID!"))?;

    Ok(meeting
        .frames
        .get(frame_index as usize)
        .ok_or(String::from("Frame index is out of bounds!"))?
        .data
        .len() as u128)
}

#[ic_cdk::query]
pub fn get_video_frame_chunk_blob(
    group_id: u128,
    meeting_id: u128,
    frame_index: u128,
    index: u128,
) -> Result<Vec<u8>, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    let meetings = MEETINGS.lock().unwrap();
    let meetings = meetings
        .get(&group_id)
        .ok_or(String::from("No meetings found on this group!"))?;

    let meeting = meetings
        .get(&meeting_id)
        .ok_or(String::from("No meeting found on this meeting ID!"))?;

    Ok(meeting
        .frames
        .get(frame_index as usize)
        .ok_or(String::from("Frame index is out of bounds!"))?
        .data
        .iter()
        .skip(index as usize * chunk::MB)
        .take(chunk::MB)
        .cloned()
        .collect())
}

#[ic_cdk::query]
pub fn get_meeting_thumbnail_size(group_id: u128, meeting_id: u128) -> Result<u128, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    let meetings = MEETINGS.lock().map_err(|err| format!("Failed to lock meeting: {}", err))?;
    let meetings = meetings
        .get(&group_id)
        .ok_or(String::from("No meetings found on this group!"))?;

    let meeting = meetings
        .get(&meeting_id)
        .ok_or(String::from("No meeting found on this meeting ID!"))?;

    Ok(meeting.thumbnail_data.len() as u128)
}

#[ic_cdk::query]
pub fn get_meeting_thumbnail_chunk_blob(
    group_id: u128,
    meeting_id: u128,
    index: u128,
) -> Result<Vec<u8>, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    let meetings = MEETINGS.lock().unwrap();
    let meetings = meetings
        .get(&group_id)
        .ok_or(String::from("No meetings found on this group!"))?;

    let meeting = meetings
        .get(&meeting_id)
        .ok_or(String::from("No meeting found on this meeting ID!"))?;

    Ok(meeting
        .thumbnail_data
        .iter()
        .skip(index as usize * chunk::MB)
        .take(chunk::MB)
        .cloned()
        .collect())
}

#[ic_cdk::query]
pub fn get_video_frame_detail(group_id: u128, meeting_id: u128, frame_index: u128) -> Result<VideoFrameHeader, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    let meetings = MEETINGS.lock().map_err(|err| format!("Failed to lock meeting: {}", err))?;
    let meetings = meetings
        .get(&group_id)
        .ok_or(String::from("No meetings found on this group!"))?;

    let meeting = meetings
        .get(&meeting_id)
        .ok_or(String::from("No meeting found on this meeting ID!"))?;

    meeting
        .frames
        .get(frame_index as usize)
        .map(VideoFrameHeader::from)
        .ok_or(String::from("Cannot find this meeting with the provided index!"))
}

#[ic_cdk::query]
pub fn get_meeting_video_frame_thumbnail_size(group_id: u128, meeting_id: u128, frame_index: u128) -> Result<u128, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    let meetings = MEETINGS.lock().map_err(|err| format!("Failed to lock meeting: {}", err))?;
    let meetings = meetings
        .get(&group_id)
        .ok_or(String::from("No meetings found on this group!"))?;

    let meeting = meetings
        .get(&meeting_id)
        .ok_or(String::from("No meeting found on this meeting ID!"))?;

    Ok(meeting
        .frames
        .get(frame_index as usize)
        .ok_or(String::from("Cannot find this meeting with the provided index!"))?
        .thumbnail_data.len() as u128)
}

#[ic_cdk::query]
pub fn get_meeting_video_frame_thumbnail_chunk_blob(
    group_id: u128,
    meeting_id: u128,
    frame_index: u128,
    index: u128,
) -> Result<Vec<u8>, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    let meetings = MEETINGS.lock().unwrap();
    let meetings = meetings
        .get(&group_id)
        .ok_or(String::from("No meetings found on this group!"))?;

    let meeting = meetings
        .get(&meeting_id)
        .ok_or(String::from("No meeting found on this meeting ID!"))?;

    Ok(meeting
        .frames
        .get(frame_index as usize)
        .ok_or(String::from("Cannot find this meeting with the provided index!"))?
        .thumbnail_data
        .iter()
        .skip(index as usize * chunk::MB)
        .take(chunk::MB)
        .cloned()
        .collect())
}

