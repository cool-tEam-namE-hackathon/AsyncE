use std::io::Cursor;

use candid::CandidType;
use mp4::{
    AacConfig, AvcConfig, HevcConfig, MediaConfig, MediaType, Mp4Config, Mp4Reader, Mp4Track,
    Mp4Writer, TrackConfig, TrackType, TtxtConfig, Vp9Config,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{globals::VIDEOS, group, user};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Video {
    pub id: String,
    pub webcam_blob: Vec<u8>,
    pub screen_blob: Vec<u8>,
}

impl Video {
    pub fn new(webcam_blob: Vec<u8>, screen_blob: Vec<u8>) -> Self {
        let cursor_webcam = Cursor::new(&webcam_blob);
        let cursor_screen = Cursor::new(&screen_blob);

        let mp4_webcam = Mp4Reader::read_header(cursor_webcam, webcam_blob.len() as u64)
            .expect("Failed to parse webcam video as mp4");

        let mp4_screen = Mp4Reader::read_header(cursor_screen, screen_blob.len() as u64)
            .expect("Failed to parse screen video as mp4");

        print_mp4_info(&mp4_webcam);
        print_mp4_info(&mp4_screen);

        Self {
            id: Uuid::new_v4().to_string(),
            webcam_blob,
            screen_blob,
        }
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

fn video_info(track: &Mp4Track) -> String {
    if track.trak.mdia.minf.stbl.stsd.avc1.is_some() {
        format!(
            "{} ({}) ({:?}), {}x{}, {} kb/s, {:.2} fps",
            track.media_type().unwrap(),
            track.video_profile().unwrap(),
            track.box_type().unwrap(),
            track.width(),
            track.height(),
            track.bitrate() / 1000,
            track.frame_rate()
        )
    } else {
        format!(
            "{} ({:?}), {}x{}, {} kb/s, {:.2} fps",
            track.media_type().unwrap(),
            track.box_type().unwrap(),
            track.width(),
            track.height(),
            track.bitrate() / 1000,
            track.frame_rate()
        )
    }
}

fn audio_info(track: &Mp4Track) -> String {
    if let Some(ref mp4a) = track.trak.mdia.minf.stbl.stsd.mp4a {
        if mp4a.esds.is_some() {
            let profile = match track.audio_profile() {
                Ok(val) => val.to_string(),
                _ => String::from("-"),
            };

            let channel_config = match track.channel_config() {
                Ok(val) => val.to_string(),
                _ => String::from("-"),
            };

            format!(
                "{} ({}) ({:?}), {} Hz, {}, {} kb/s",
                track.media_type().unwrap(),
                profile,
                track.box_type().unwrap(),
                track.sample_freq_index().unwrap().freq(),
                channel_config,
                track.bitrate() / 1000
            )
        } else {
            format!(
                "{} ({:?}), {} kb/s",
                track.media_type().unwrap(),
                track.box_type().unwrap(),
                track.bitrate() / 1000
            )
        }
    } else {
        String::from("mp4a box not found")
    }
}

fn subtitle_info(track: &Mp4Track) -> String {
    if track.trak.mdia.minf.stbl.stsd.tx3g.is_some() {
        format!(
            "{} ({:?})",
            track.media_type().unwrap(),
            track.box_type().unwrap(),
        )
    } else {
        String::from("tx3g box not found")
    }
}

fn creation_time(creation_time: u64) -> u64 {
    // convert from MP4 epoch (1904-01-01) to Unix epoch (1970-01-01)
    if creation_time >= 2082844800 {
        creation_time - 2082844800
    } else {
        creation_time
    }
}

fn print_mp4_info(mp4: &Mp4Reader<Cursor<&Vec<u8>>>) {
    ic_cdk::println!("File:");
    ic_cdk::println!("  file size:          {}", mp4.size());
    ic_cdk::println!("  major_brand:        {}", mp4.major_brand());
    let mut compatible_brands = String::new();
    for brand in mp4.compatible_brands().iter() {
        compatible_brands.push_str(&brand.to_string());
        compatible_brands.push(' ');
    }
    ic_cdk::println!("  compatible_brands:  {}\n", compatible_brands);

    ic_cdk::println!("Movie:");
    ic_cdk::println!("  version:        {}", mp4.moov.mvhd.version);
    ic_cdk::println!(
        "  creation time:  {}",
        creation_time(mp4.moov.mvhd.creation_time)
    );
    ic_cdk::println!("  duration:       {:?}", mp4.duration());
    ic_cdk::println!("  fragments:      {:?}", mp4.is_fragmented());
    ic_cdk::println!("  timescale:      {:?}\n", mp4.timescale());

    ic_cdk::println!("Found {} Tracks", mp4.tracks().len());
    for track in mp4.tracks().values() {
        let media_info = match track.track_type().unwrap() {
            TrackType::Video => video_info(track),
            TrackType::Audio => audio_info(track),
            TrackType::Subtitle => subtitle_info(track),
        };

        ic_cdk::println!(
            "  Track: #{}({}) {}: {}",
            track.track_id(),
            track.language(),
            track.track_type().unwrap(),
            media_info
        );
    }
}

fn assert_check_group(group_id: &str) {
    let group = match group::get_group(group_id.to_owned()) {
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

#[ic_cdk::query]
pub fn get_videos(group_id: String) -> Vec<Video> {
    user::assert_user_logged_in();
    assert_check_group(&group_id);

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
pub fn add_video(group_id: String, webcam_blob: Vec<u8>, screen_blob: Vec<u8>) {
    user::assert_user_logged_in();
    assert_check_group(&group_id);

    let video = Video::new(webcam_blob, screen_blob);
    VIDEOS.with_borrow_mut(|videos| {
        videos
            .entry(group_id)
            .or_default()
            .insert(video.id.clone(), video)
    });
}

#[ic_cdk::update]
pub fn concat_video(
    group_id: String,
    video_id: String,
    webcam_blob: Vec<u8>,
    screen_blob: Vec<u8>,
) {
    user::assert_user_logged_in();
    assert_check_group(&group_id);

    VIDEOS.with_borrow_mut(|videos| {
        let videos = match videos.get_mut(&group_id) {
            Some(video) => video,
            None => panic!("No videos found on this group!"),
        };

        let video = match videos.get_mut(&video_id) {
            Some(video) => video,
            None => panic!("No video found on this video ID!"),
        };

        concat_mp4(&mut video.webcam_blob, &webcam_blob);
        concat_mp4(&mut video.screen_blob, &screen_blob);
    });
}
