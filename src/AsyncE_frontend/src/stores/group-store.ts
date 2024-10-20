import { ref } from "vue";

import { defineStore } from "pinia";

import { useUserStore } from "@stores/user-store";

import { storeToRefs } from "pinia";

import { Group, VideoList } from "@/types/api/model";
import { MB } from "@/data/user-constants";
import { GroupQueryResponse } from "@declarations/AsyncE_backend/AsyncE_backend.did";
import { blobToURL, validateResponse } from "@/utils/helpers";

export const useGroupStore = defineStore("group", () => {
    const { actor } = storeToRefs(useUserStore());

    const groupPicture = ref<string>("");
    const groupList = ref<Group[]>([]);
    const currentGroup = ref<Group>();
    const uploadVideoProgress = ref<number>(0);
    const videosList = ref<VideoList[]>([]);

    function convertGroupFromResponse(groupResponse: GroupQueryResponse) {
        return {
            id: groupResponse.id,
            name: groupResponse.name,
            owner: groupResponse.name,
            users: groupResponse.users,
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
        const response = await actor.value?.get_group_profile_picture_size(
            groupId,
        );
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
            await actor.value?.upload_group_profile_picture(groupId, chunk);
        }
    }

    async function getGroup(id: bigint) {
        const response = await actor.value?.get_group(id);

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
        //                     ?.get_group_profile_picture_chunk_blob(
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

    async function addVideo(data: Uint8Array, groupId: string, title: string) {
        console.log("data", data);
        console.log("groupId", groupId);
        console.log("title", title);

        const videoId = await actor.value?.create_video(BigInt(groupId), title);

        const okVideoId = validateResponse(videoId);

        const totalChunks = Math.ceil(data.length / MB);

        let totalBytesUploaded = 0;

        for (let i = 0; i < totalChunks; ++i) {
            const start = i * MB;
            const end = Math.min(start + MB, data.length);
            const chunk = data.slice(start, end);
            validateResponse(
                await actor.value?.upload_video(
                    BigInt(groupId),
                    okVideoId,
                    chunk,
                    i === totalChunks - 1,
                ),
            );

            totalBytesUploaded += chunk.length;
            uploadVideoProgress.value =
                (totalBytesUploaded / data.length) * 100;
        }

        console.log("all good bitch");
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

    async function getAllVideos(groupId: string) {
        const response = await actor.value?.get_videos(BigInt(groupId));

        const okResponse = validateResponse(response);

        videosList.value = [];

        const videoPromises = okResponse.map(async (video) => {
            const videoSize = await actor.value?.get_video_size(
                BigInt(groupId),
                video.id,
            );

            const okVideoSize = validateResponse(videoSize);

            const okVideoBlobSize = Number(okVideoSize);
            const okVideoData = new Uint8Array(okVideoBlobSize);

            const chunkPromises = [];
            for (let j = 0; j < Math.ceil(okVideoBlobSize / MB); ++j) {
                chunkPromises.push(
                    actor.value
                        ?.get_video_chunk_blob(
                            BigInt(groupId),
                            video.id,
                            BigInt(j),
                        )
                        .then((chunk) => {
                            const okChunk = validateResponse(chunk);
                            okVideoData.set(okChunk, j * MB);
                        }),
                );
            }

            await Promise.all(chunkPromises);

            const url = blobToURL(okVideoData);

            return { video, url };
        });

        videoPromises.forEach((promise) => {
            promise.then((result) => {
                console.log("pushed");
                videosList.value.push(result);
            });
        });

        await Promise.allSettled(videoPromises);
    }

    return {
        currentGroup,
        groupList,
        groupPicture,
        uploadVideoProgress,
        videosList,

        addVideo,
        getAllGroups,
        getAllVideos,
        getGroup,
        getInvites,
        handleInvitation,
        createGroup,
        inviteUser,
    };
});
