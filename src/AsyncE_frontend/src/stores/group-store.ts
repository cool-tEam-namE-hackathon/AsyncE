import { ref } from "vue";

import { defineStore } from "pinia";
import { storeToRefs } from "pinia";

import {
    GroupQueryResponse,
    MeetingHeader,
} from "@declarations/AsyncE_backend/AsyncE_backend.did";

import { useUserStore } from "@stores/user-store";

import { MB } from "@/data/user-constants";
import { Group } from "@/types/api/model";
import { blobToURL, validateResponse } from "@/utils/helpers";

export const useGroupStore = defineStore("group", () => {
    const { actor } = storeToRefs(useUserStore());

    const groupPicture = ref<string>("");
    const groupList = ref<Group[]>([]);
    const currentGroup = ref<Group>();
    const uploadVideoProgress = ref<number>(0);

    const meetingList = ref<MeetingHeader[]>([]);
    const meetingDetail = ref<MeetingHeader>();
    const videoThumbnail = ref<string[]>([]);

    const meetingVideo = ref<string>("");
    const selectedVideo = ref<string>("");

    function convertGroupFromResponse(groupResponse: GroupQueryResponse) {
        return {
            id: groupResponse.id,
            name: groupResponse.name,
            owner: groupResponse.name,
            members: groupResponse.members,
            profile_picture_blob: new Uint8Array(),
        };
    }

    async function getAllGroups() {
        const response = await actor.value?.get_all_groups();
        const okResponse = validateResponse(response);

        const groupFetchPromises = okResponse.map(
            async (groupResponse, index) => {
                const group = convertGroupFromResponse(groupResponse);
                const profilePictureData = await fetchGroupProfilePicture(
                    group.id,
                );

                group.profile_picture_blob = profilePictureData;
                groupList.value[index] = group;
            },
        );

        await Promise.all(groupFetchPromises);
    }

    async function fetchGroupProfilePicture(groupId: bigint) {
        const response =
            await actor.value?.get_group_profile_picture_size(groupId);
        const profilePictureSize = Number(validateResponse(response));
        const profilePictureData = new Uint8Array(profilePictureSize);

        const chunkPromises = Array.from(
            { length: Math.ceil(profilePictureSize / MB) },
            (_, i) =>
                actor.value
                    ?.get_group_profile_picture_chunk_blob(groupId, BigInt(i))
                    .then((chunk) => {
                        const okChunk = validateResponse(chunk);
                        profilePictureData.set(okChunk, i * MB);
                    }),
        );

        await Promise.all(chunkPromises);
        return profilePictureData;
    }

    async function createGroup({
        name,
        picture,
    }: {
        name: string;
        picture: Uint8Array;
    }) {
        const response = await actor.value?.create_group(name);

        const groupId = validateResponse(response);

        for (let i = 0; i < Math.ceil(picture.length / MB); ++i) {
            const start = i * MB;
            const end = Math.min(start + MB, picture.length);
            const chunk = picture.slice(start, end);
            await actor.value?.upload_group_profile_picture(
                groupId,
                chunk,
                BigInt(i),
                BigInt(picture.length),
            );
        }
    }

    async function getGroup(id: string) {
        const response = await actor.value?.get_group(BigInt(id));

        const okResponse = validateResponse(response);

        if (!okResponse[0]) return;

        currentGroup.value = convertGroupFromResponse(okResponse[0]);

        // await actor.value
        //     ?.get_group_profile_picture_size(currentGroup.value.id)
        //     .then(async (groupPictureBlobSizeBigInt) => {
        //         const profilePictureBlobSize = Number(
        //             groupPictureBlobSizeBigInt,
        //         );
        //         const profilePictureData = new Uint8Array(
        //             profilePictureBlobSize,
        //         );

        //         const promises = [];
        //         for (
        //             let i = 0;
        //             i < Math.ceil(profilePictureBlobSize / MB);
        //             ++i
        //         ) {
        //             promises.push(
        //                 actor.value
        //                     ?._profile_picture_chunk_blob(
        //                         currentGroup.value!.id,
        //                         BigInt(i),
        //                     )
        //                     .then((chunk) => {
        //                         const okChunk = validateResponse(chunk);
        //                         profilePictureData.set(okChunk, i * MB);
        //                     }),
        //             );
        //         }

        //         await Promise.all(promises);

        //         currentGroup.value!.profile_picture_blob = profilePictureData;
        //     });

        return okResponse[0];
    }

    async function uploadVideo(
        data: Uint8Array,
        groupId: string,
        meetingId: string,
        title: string,
        subtitle: boolean,
    ) {
        const totalChunks = Math.ceil(data.length / MB);
        const uuid = crypto.randomUUID();
        let totalBytesUploaded = 0;

        const uploadPromises = [];

        for (let i = 0; i < totalChunks; ++i) {
            const start = i * MB;
            const end = Math.min(start + MB, data.length);
            const chunk = data.slice(start, end);

            const uploadPromise = actor.value
                ?.upload_video(
                    BigInt(groupId),
                    BigInt(meetingId),
                    chunk,
                    i === totalChunks - 1,
                    title,
                    uuid,
                    BigInt(i),
                    BigInt(data.length),
                    subtitle,
                )
                .then((response) => {
                    validateResponse(response);
                    totalBytesUploaded += chunk.length;
                    uploadVideoProgress.value =
                        (totalBytesUploaded / data.length) * 100;
                });

            uploadPromises.push(uploadPromise);
        }

        await Promise.all(uploadPromises);

        console.log("All chunks uploaded successfully!");
    }

    async function getMeetingVideo(groupId: string, meetingId: string) {
        const response = await actor.value?.get_video_meeting_size(
            BigInt(groupId),
            BigInt(meetingId),
        );

        const videoMeetingSize = Number(validateResponse(response));

        const videoMeetingData = new Uint8Array(videoMeetingSize);

        const videoPromises = [];

        for (let i = 0; i < Math.ceil(videoMeetingSize / MB); ++i) {
            const videoPromise = actor.value
                ?.get_video_meeting_chunk_blob(
                    BigInt(groupId),
                    BigInt(meetingId),
                    BigInt(i),
                )
                .then((chunk) => {
                    const okChunk = validateResponse(chunk);
                    videoMeetingData.set(okChunk, i * MB);
                });
            videoPromises.push(videoPromise);
        }

        await Promise.all(videoPromises);
        meetingVideo.value = blobToURL(videoMeetingData);
    }

    async function getVideo(groupId: string, meetingId: string, index: number) {
        const response = await actor.value?.get_video_frame_size(
            BigInt(groupId),
            BigInt(meetingId),
            BigInt(index),
        );

        const videoSize = Number(validateResponse(response));

        const videoData = new Uint8Array(videoSize);

        const videoPromises = [];

        for (let i = 0; i < Math.ceil(videoSize / MB); ++i) {
            const videoPromise = actor.value
                ?.get_video_frame_chunk_blob(
                    BigInt(groupId),
                    BigInt(meetingId),
                    BigInt(index),
                    BigInt(i),
                )
                .then((chunk) => {
                    const okChunk = validateResponse(chunk);
                    videoData.set(okChunk, i * MB);
                });
            videoPromises.push(videoPromise);
        }

        await Promise.all(videoPromises);
        selectedVideo.value = blobToURL(videoData);
    }

    async function inviteUser(id: bigint, name: string) {
        const response = await actor.value?.invite_user(id, name);

        validateResponse(response);
    }

    async function getInvites() {
        const response = await actor.value?.get_self_group_invites();

        const okResponse = validateResponse(response);

        return okResponse;
    }

    async function handleInvitation(groupId: bigint, invitation: boolean) {
        const response = await actor.value?.update_group_invite(
            groupId,
            invitation,
        );

        validateResponse(response);
    }

    async function getAllThumbnails(groupId: string) {
        const totalFrames = meetingDetail.value?.frames_count;
        const meetingId = meetingDetail.value?.id;

        if (!meetingId) return;

        for (let i = 0; i < Number(totalFrames); ++i) {
            const thumbnailSize =
                await actor.value?.get_meeting_video_frame_thumbnail_size(
                    BigInt(groupId),
                    meetingId,
                    BigInt(i),
                );

            const okThumbnailSize = validateResponse(thumbnailSize);

            const okThumbnailBlobSize = Number(okThumbnailSize);
            const okThumbnailData = new Uint8Array(okThumbnailBlobSize);

            const chunkPromises = [];
            for (let j = 0; j < Math.ceil(okThumbnailBlobSize / MB); ++j) {
                chunkPromises.push(
                    actor.value
                        ?.get_meeting_video_frame_thumbnail_chunk_blob(
                            BigInt(groupId),
                            meetingId,
                            BigInt(i),
                            BigInt(j),
                        )
                        .then((chunk) => {
                            const okChunk = validateResponse(chunk);
                            okThumbnailData.set(okChunk, j * MB);
                        }),
                );
            }

            await Promise.all(chunkPromises);

            videoThumbnail.value.push(blobToURL(okThumbnailData));
        }
    }

    async function createMeeting(groupId: string, meetingName: string) {
        const response = await actor.value?.create_meeting(
            BigInt(groupId),
            meetingName,
        );

        const okResponse = validateResponse(response);

        return okResponse;
    }

    async function getAllMeetings(groupId: string) {
        const response = await actor.value?.get_meetings(BigInt(groupId));

        const okResponse = validateResponse(response);

        meetingList.value = okResponse;

        return okResponse;
    }

    async function getMeetingDetail(groupId: string, meetingId: string) {
        const response = await actor.value?.get_meeting_detail(
            BigInt(groupId),
            BigInt(meetingId),
        );

        const okResponse = validateResponse(response);

        meetingDetail.value = okResponse;

        return okResponse;
    }

    async function getChats(groupId: string) {
        const response = await actor.value?.get_chats(BigInt(groupId));

        const okResponse = validateResponse(response);

        return okResponse;
    }

    return {
        currentGroup,
        groupList,
        groupPicture,
        uploadVideoProgress,
        meetingList,
        meetingDetail,
        videoThumbnail,
        meetingVideo,
        selectedVideo,

        uploadVideo,
        getAllGroups,
        getAllMeetings,
        getVideo,
        createMeeting,
        getMeetingDetail,
        getAllThumbnails,
        getChats,
        getMeetingVideo,
        getGroup,
        getInvites,
        handleInvitation,
        createGroup,
        inviteUser,
    };
});
