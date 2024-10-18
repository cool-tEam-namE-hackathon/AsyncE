use std::io::Cursor;

use candid::CandidType;
use mp4::{
    AacConfig, AvcConfig, HevcConfig, MediaConfig, MediaType, Mp4Config, Mp4Reader, Mp4Writer,
    TrackConfig, TtxtConfig, Vp9Config,
};
use serde::Deserialize;

use crate::{
    chunk,
    globals::{VIDEOS, VIDEO_UPLOADS},
    group,
    primary_key::{self, PrimaryKeyType},
    user,
};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Video {
    pub id: u128,
    pub data: Vec<u8>,
    pub thumbnail_data: Vec<u8>,
    pub title: String,
    pub frames: Vec<VideoFrame>,
    pub created_by: String,
    pub created_time_unix: u128,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct VideoHeader {
    pub id: u128,
    pub title: String,
    pub created_by: String,
    pub frames_count: u128,
    pub created_time_unix: u128,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct VideoFrame {
    pub data: Vec<u8>,
    pub title: String,
    pub created_by: String,
    pub created_time_unix: u128,
}

impl VideoFrame {
    fn new(username: String, title: String) -> Self {
        Self {
            data: Vec::new(),
            title,
            created_by: username,
            created_time_unix: ic_cdk::api::time() as u128,
        }
    }
}

impl Video {
    pub fn new(username: String, title: String) -> Self {
        Self {
            id: primary_key::get_primary_key(PrimaryKeyType::Video),
            data: Vec::new(),
            thumbnail_data: Vec::new(),
            frames: Vec::from([VideoFrame::new(username.clone(), title.clone())]),
            title,
            created_by: username,
            created_time_unix: ic_cdk::api::time() as u128,
        }
    }

    pub fn set_data(&mut self, data: Vec<u8>) -> Result<(), String> {
        Mp4Reader::read_header(Cursor::new(&data), data.len() as u64).map_err(|error| {
            format!(
                "Invalid MP4 Data while trying to set data for video: {}!\n{}",
                error,
                hex::encode(&data),
            )
        })?;

        self.data = data;

        Ok(())
    }
}

impl From<&Video> for VideoHeader {
    fn from(value: &Video) -> Self {
        Self {
            id: value.id,
            title: value.title.clone(),
            created_by: value.created_by.clone(),
            frames_count: value.frames.len() as u128,
            created_time_unix: value.created_time_unix,
        }
    }
}

fn assert_check_group(group_id: u128) -> Result<(), String> {
    let group =
        group::get_group(group_id)?.ok_or(String::from("Group with this ID is not found"))?;

    let name = user::get_selfname_force()?;

    if group.owner != name && !group.users.contains(&name) {
        return Err(String::from("Current user does not belong to this group!"));
    }

    Ok(())
}

fn add_mp4_track<T: AsRef<[u8]>>(
    mp4_reader: &mut Mp4Reader<&mut Cursor<T>>,
    mp4_writer: &mut Mp4Writer<Cursor<Vec<u8>>>,
) -> Result<(), String> {
    for track in mp4_reader.tracks().values() {
        let media_conf = track
            .media_type()
            .map_err(|_| String::from("Cannot find media type on the track"))?;

        let media_conf = match media_conf {
            MediaType::H264 => MediaConfig::AvcConfig(AvcConfig {
                width: track.width(),
                height: track.height(),
                seq_param_set: track
                    .sequence_parameter_set()
                    .map_err(|_| String::from("Cannot find sequence_parameter_set on the track"))?
                    .to_vec(),

                pic_param_set: track
                    .picture_parameter_set()
                    .map_err(|_| String::from("Cannot find picture_parameter_set on the track"))?
                    .to_vec(),
            }),

            MediaType::H265 => MediaConfig::HevcConfig(HevcConfig {
                width: track.width(),
                height: track.height(),
            }),

            MediaType::VP9 => MediaConfig::Vp9Config(Vp9Config {
                width: track.width(),
                height: track.height(),
            }),

            MediaType::AAC => MediaConfig::AacConfig(AacConfig {
                bitrate: track.bitrate(),
                profile: track
                    .audio_profile()
                    .map_err(|_| String::from("Cannot find audio_profile on the track"))?,

                freq_index: track
                    .sample_freq_index()
                    .map_err(|_| String::from("Cannot find sample_freq_index on the track"))?,

                chan_conf: track
                    .channel_config()
                    .map_err(|_| String::from("Cannot find channel_config on the track"))?,
            }),

            MediaType::TTXT => MediaConfig::TtxtConfig(TtxtConfig {}),
        };

        let track_conf = TrackConfig {
            track_type: track
                .track_type()
                .map_err(|_| String::from("Cannot find track_type on the track"))?,

            timescale: track.timescale(),
            language: track.language().to_string(),
            media_conf,
        };

        mp4_writer
            .add_track(&track_conf)
            .map_err(|_| String::from("Cannot add track to the mp4 writer!"))?;
    }

    for track_id in mp4_reader.tracks().keys().copied().collect::<Vec<u32>>() {
        let sample_count = mp4_reader
            .sample_count(track_id)
            .map_err(|_| String::from("Cannot get sample_count from the MP4 reader"))?;

        for sample_idx in 0..sample_count {
            let sample_id = sample_idx + 1;
            let sample = mp4_reader
                .read_sample(track_id, sample_id)
                .map_err(|_| {
                    format!(
                        "Cannot read the mp4 sample on track id {} and sample id {}",
                        track_id, sample_id
                    )
                })?
                .ok_or(format!(
                    "MP4 sample is null on track id {} and sample id {}",
                    track_id, sample_id
                ))?;

            mp4_writer
                .write_sample(track_id, &sample)
                .map_err(|_| String::from("Cannot write sample to the mp4 writer!"))?;
        }
    }

    Ok(())
}

fn concat_mp4(self_blob_data: &mut Vec<u8>, blob_data: &[u8]) -> Result<(), String> {
    let mut mp4_reader1_cursor = Cursor::new(&self_blob_data);
    let mut mp4_reader1 =
        Mp4Reader::read_header(&mut mp4_reader1_cursor, self_blob_data.len() as u64)
            .map_err(|_| String::from("Invalid MP4 on existing data!"))?;

    let mut mp4_reader2_cursor = Cursor::new(&blob_data);
    let mut mp4_reader2 = Mp4Reader::read_header(&mut mp4_reader2_cursor, blob_data.len() as u64)
        .map_err(|_| String::from("Invalid MP4 data!"))?;

    let output = Cursor::new(Vec::new());
    let mp4_config = Mp4Config {
        major_brand: *mp4_reader1.major_brand(),
        minor_version: mp4_reader1.minor_version(),
        compatible_brands: mp4_reader1.compatible_brands().to_vec(),
        timescale: mp4_reader1.timescale(),
    };

    let mut mp4_writer = mp4::Mp4Writer::write_start(output, &mp4_config).map_err(|_| {
        String::from("Cannot write MP4 data start to output given the existing config!")
    })?;

    add_mp4_track(&mut mp4_reader1, &mut mp4_writer)?;
    add_mp4_track(&mut mp4_reader2, &mut mp4_writer)?;

    mp4_writer
        .write_end()
        .map_err(|_| String::from("Cannot write MP4 data end to output!"))?;

    *self_blob_data = mp4_writer.into_writer().into_inner();

    Ok(())
}

#[ic_cdk::query]
pub fn get_videos(group_id: u128) -> Result<Vec<VideoHeader>, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    VIDEOS.with_borrow(|videos| {
        Ok(videos
            .get(&group_id)
            .cloned()
            .unwrap_or_default()
            .values()
            .map(VideoHeader::from)
            .collect::<Vec<_>>())
    })
}

#[ic_cdk::query]
pub fn get_video_detail(group_id: u128, video_id: u128) -> Result<VideoHeader, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    VIDEOS.with_borrow(|videos| {
        videos
            .get(&group_id)
            .ok_or(String::from("No videos found on this group!"))?
            .get(&video_id)
            .ok_or(String::from("No video found on this video ID!"))
            .map(VideoHeader::from)
    })
}

#[ic_cdk::update]
pub fn create_video(group_id: u128, title: String) -> Result<u128, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    let selfname = user::get_selfname_force()?;
    let video = Video::new(selfname.clone(), title.clone());
    let video_id = video.id;

    VIDEOS.with_borrow_mut(|videos| videos.entry(group_id).or_default().insert(video.id, video));
    VIDEO_UPLOADS.with_borrow_mut(|video_uploads| video_uploads.insert(video_id, Vec::new()));

    Ok(video_id)
}

#[ic_cdk::update]
pub fn create_video_frame(group_id: u128, video_id: u128, title: String) -> Result<(), String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    let selfname = user::get_selfname_force()?;

    VIDEOS.with_borrow_mut(|videos| {
        let videos = videos
            .get_mut(&group_id)
            .ok_or(String::from("No videos found on this group!"))?;

        let video = videos
            .get_mut(&video_id)
            .ok_or(String::from("No video found on this video ID!"))?;

        video.frames.push(VideoFrame::new(selfname, title));

        VIDEO_UPLOADS.with_borrow_mut(|video_uploads| video_uploads.insert(video_id, Vec::new()));

        Ok(())
    })
}

#[ic_cdk::update]
pub fn upload_video(
    group_id: u128,
    video_id: u128,
    data: Vec<u8>,
    finish: bool,
) -> Result<(), String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    VIDEOS.with_borrow_mut(|videos| {
        let videos = videos
            .get_mut(&group_id)
            .ok_or(String::from("No videos found on this group!"))?;

        let video = videos
            .get_mut(&video_id)
            .ok_or(String::from("No video found on this video ID!"))?;

        VIDEO_UPLOADS.with_borrow_mut(|video_uploads| {
            video_uploads
                .get_mut(&video_id)
                .ok_or(String::from(
                    "Cannot find existing upload process with given ID",
                ))?
                .extend(data);

            if finish {
                let data = video_uploads.remove(&video_id).ok_or(String::from(
                    "Cannot find existing upload process with given ID (This should never happen though)",
                ))?;

                if !video.data.is_empty() {
                    concat_mp4(&mut video.data, &data)?;
                } else {
                    video.set_data(data.clone())?;
                }

                let video_frame = video.frames.last_mut().ok_or(String::from(
                    "Cannot find latest frame (This should never happen though)",
                ))?;
                video_frame.data = data;
            }

            Ok(())
        })
    })
}

#[ic_cdk::query]
pub fn get_video_size(group_id: u128, video_id: u128) -> Result<u128, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    VIDEOS.with_borrow_mut(|videos| {
        let videos = videos
            .get_mut(&group_id)
            .ok_or(String::from("No videos found on this group!"))?;

        let video = videos
            .get_mut(&video_id)
            .ok_or(String::from("No video found on this video ID!"))?;

        Ok(video.data.len() as u128)
    })
}

#[ic_cdk::query]
pub fn get_video_chunk_blob(
    group_id: u128,
    video_id: u128,
    index: u128,
) -> Result<Vec<u8>, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    VIDEOS.with_borrow_mut(|videos| {
        let videos = videos
            .get_mut(&group_id)
            .ok_or(String::from("No videos found on this group!"))?;

        let video = videos
            .get_mut(&video_id)
            .ok_or(String::from("No video found on this video ID!"))?;

        Ok(video
            .data
            .iter()
            .skip(index as usize * chunk::MB)
            .take(chunk::MB)
            .cloned()
            .collect())
    })
}

#[ic_cdk::query]
pub fn get_video_frame_size(
    group_id: u128,
    video_id: u128,
    frame_index: u128,
) -> Result<u128, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    VIDEOS.with_borrow_mut(|videos| {
        let videos = videos
            .get_mut(&group_id)
            .ok_or(String::from("No videos found on this group!"))?;

        let video = videos
            .get_mut(&video_id)
            .ok_or(String::from("No video found on this video ID!"))?;

        Ok(video
            .frames
            .get(frame_index as usize)
            .ok_or(String::from("Frame index is out of bounds!"))?
            .data
            .len() as u128)
    })
}

#[ic_cdk::query]
pub fn get_video_frame_chunk_blob(
    group_id: u128,
    video_id: u128,
    frame_index: u128,
    index: u128,
) -> Result<Vec<u8>, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    VIDEOS.with_borrow_mut(|videos| {
        let videos = videos
            .get_mut(&group_id)
            .ok_or(String::from("No videos found on this group!"))?;

        let video = videos
            .get_mut(&video_id)
            .ok_or(String::from("No video found on this video ID!"))?;

        Ok(video
            .frames
            .get(frame_index as usize)
            .ok_or(String::from("Frame index is out of bounds!"))?
            .data
            .iter()
            .skip(index as usize * chunk::MB)
            .take(chunk::MB)
            .cloned()
            .collect())
    })
}

#[ic_cdk::query]
pub fn get_video_thumbnaiL_size(group_id: u128, video_id: u128) -> Result<u128, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    VIDEOS.with_borrow_mut(|videos| {
        let videos = videos
            .get_mut(&group_id)
            .ok_or(String::from("No videos found on this group!"))?;

        let video = videos
            .get_mut(&video_id)
            .ok_or(String::from("No video found on this video ID!"))?;

        Ok(video.thumbnail_data.len() as u128)
    })
}

#[ic_cdk::query]
pub fn get_video_thumbnail_chunk_blob(
    group_id: u128,
    video_id: u128,
    index: u128,
) -> Result<Vec<u8>, String> {
    user::assert_user_logged_in()?;
    assert_check_group(group_id)?;

    VIDEOS.with_borrow_mut(|videos| {
        let videos = videos
            .get_mut(&group_id)
            .ok_or(String::from("No videos found on this group!"))?;

        let video = videos
            .get_mut(&video_id)
            .ok_or(String::from("No video found on this video ID!"))?;

        Ok(video
            .thumbnail_data
            .iter()
            .skip(index as usize * chunk::MB)
            .take(chunk::MB)
            .cloned()
            .collect())
    })
}
