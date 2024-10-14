use std::io::Cursor;

use candid::CandidType;
use mp4::{
    AacConfig, AvcConfig, HevcConfig, MediaConfig, MediaType, Mp4Config, Mp4Reader, Mp4Writer,
    TrackConfig, TtxtConfig, Vp9Config,
};
use serde::Deserialize;

use crate::{
    globals::{VIDEOS, VIDEO_UPLOADS},
    group,
    primary_key::{self, PrimaryKeyType},
    user,
};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Video {
    pub id: u128,
    pub data: Vec<u8>,
    pub username: String,
    pub created_time_unix: u128,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct VideoFrames {
    pub id: u128,
    pub data: Vec<u8>,
    pub username: String,
    pub created_time_unix: u128,
}

impl Video {
    pub fn new(username: String) -> Self {
        Self {
            id: primary_key::get_primary_key(PrimaryKeyType::Video),
            data: Vec::new(),
            username,
            created_time_unix: ic_cdk::api::time() as u128,
        }
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        Mp4Reader::read_header(Cursor::new(&data), data.len() as u64).unwrap();
        self.data = data;
    }
}

fn assert_check_group(group_id: u128) {
    let group = match group::get_group(group_id) {
        Some(group) => group,
        None => panic!("Group not found!"),
    };

    let name = match user::get_selfname() {
        Some(name) => name,
        None => panic!("Current user does not have a name!"),
    };

    if group.owner != name && !group.users.contains(&name) {
        panic!("Current user does not belong to this group!")
    }
}

fn add_mp4_track<T: AsRef<[u8]>>(
    mp4_reader: &mut Mp4Reader<&mut Cursor<T>>,
    mp4_writer: &mut Mp4Writer<Cursor<Vec<u8>>>,
) {
    for track in mp4_reader.tracks().values() {
        let media_conf = match track.media_type().unwrap() {
            MediaType::H264 => MediaConfig::AvcConfig(AvcConfig {
                width: track.width(),
                height: track.height(),
                seq_param_set: track.sequence_parameter_set().unwrap().to_vec(),
                pic_param_set: track.picture_parameter_set().unwrap().to_vec(),
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
                profile: track.audio_profile().unwrap(),
                freq_index: track.sample_freq_index().unwrap(),
                chan_conf: track.channel_config().unwrap(),
            }),

            MediaType::TTXT => MediaConfig::TtxtConfig(TtxtConfig {}),
        };

        let track_conf = TrackConfig {
            track_type: track.track_type().unwrap(),
            timescale: track.timescale(),
            language: track.language().to_string(),
            media_conf,
        };

        mp4_writer.add_track(&track_conf).unwrap();
    }

    for track_id in mp4_reader.tracks().keys().copied().collect::<Vec<u32>>() {
        let sample_count = mp4_reader.sample_count(track_id).unwrap();
        for sample_idx in 0..sample_count {
            let sample_id = sample_idx + 1;
            let sample = mp4_reader
                .read_sample(track_id, sample_id)
                .unwrap()
                .unwrap();
            mp4_writer.write_sample(track_id, &sample).unwrap();
        }
    }
}

fn concat_mp4(self_blob_data: &mut Vec<u8>, blob_data: &[u8]) {
    let mut mp4_reader1_cursor = Cursor::new(&self_blob_data);
    let mut mp4_reader1 =
        Mp4Reader::read_header(&mut mp4_reader1_cursor, self_blob_data.len() as u64).unwrap();

    let mut mp4_reader2_cursor = Cursor::new(&blob_data);
    let mut mp4_reader2 =
        Mp4Reader::read_header(&mut mp4_reader2_cursor, blob_data.len() as u64).unwrap();

    let output = Cursor::new(Vec::new());
    let mp4_config = Mp4Config {
        major_brand: *mp4_reader1.major_brand(),
        minor_version: mp4_reader1.minor_version(),
        compatible_brands: mp4_reader1.compatible_brands().to_vec(),
        timescale: mp4_reader1.timescale(),
    };
    let mut mp4_writer = mp4::Mp4Writer::write_start(output, &mp4_config).unwrap();

    add_mp4_track(&mut mp4_reader1, &mut mp4_writer);
    add_mp4_track(&mut mp4_reader2, &mut mp4_writer);

    mp4_writer.write_end().unwrap();
    *self_blob_data = mp4_writer.into_writer().into_inner();
}

#[ic_cdk::query]
pub fn get_videos(group_id: u128) -> Vec<Video> {
    user::assert_user_logged_in();
    assert_check_group(group_id);

    VIDEOS.with_borrow(|videos| {
        videos
            .get(&group_id)
            .cloned()
            .unwrap_or_default()
            .values()
            .cloned()
            .collect::<Vec<_>>()
    })
}

#[ic_cdk::update]
pub fn create_video(group_id: u128) -> u128 {
    user::assert_user_logged_in();
    assert_check_group(group_id);

    let selfname = user::get_selfname().unwrap();
    let video = Video::new(selfname);
    let video_id = video.id;
    VIDEOS.with_borrow_mut(|videos| videos.entry(group_id).or_default().insert(video.id, video));
    VIDEO_UPLOADS.with_borrow_mut(|video_uploads| video_uploads.insert(video_id, Vec::new()));
    video_id
}

#[ic_cdk::update]
pub fn upload_video(group_id: u128, video_id: u128, data: Vec<u8>, finish: bool) {
    user::assert_user_logged_in();
    assert_check_group(group_id);

    VIDEO_UPLOADS.with_borrow_mut(|video_uploads| {
        video_uploads.get_mut(&video_id).unwrap().extend(data);

        if finish {
            let data = video_uploads.remove(&video_id).unwrap();

            VIDEOS.with_borrow_mut(|videos| {
                videos
                    .get_mut(&group_id)
                    .unwrap()
                    .get_mut(&video_id)
                    .unwrap()
                    .set_data(data)
            })
        }
    });
}

#[ic_cdk::update]
pub fn concat_video(group_id: u128, video_id: u128, data: Vec<u8>) {
    user::assert_user_logged_in();
    assert_check_group(group_id);

    VIDEOS.with_borrow_mut(|videos| {
        let videos = match videos.get_mut(&group_id) {
            Some(video) => video,
            None => panic!("No videos found on this group!"),
        };

        let video = match videos.get_mut(&video_id) {
            Some(video) => video,
            None => panic!("No video found on this video ID!"),
        };

        concat_mp4(&mut video.data, &data);
    });
}
