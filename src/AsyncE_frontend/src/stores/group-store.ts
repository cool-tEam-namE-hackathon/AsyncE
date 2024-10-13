import { ref } from "vue";

import { defineStore } from "pinia";

import { useUserStore } from "@stores/user-store";

import { storeToRefs } from "pinia";

import { Group } from "@/types/api/model";
import { MB } from "@/data/user-constants";

export const useGroupStore = defineStore("group", () => {
    const { actor } = storeToRefs(useUserStore());

    const groupPicture = ref<string>("");
    const groupList = ref<Group[]>([]);
    const currentGroup = ref<Group>();

    async function getAllGroups() {
        const response = await actor.value?.get_all_groups();
        if (response) {
            groupList.value = [];

            for (let i = 0; i < response.length; ++i) {
                const groupResponse = response[i];
                const group: Group = {
                    id: groupResponse.id,
                    name: groupResponse.name,
                    owner: groupResponse.name,
                    users: groupResponse.users,
                    profile_picture_blob: new Uint8Array(),
                };
                groupList.value.push(group);

                actor.value?.get_group_profile_picture_size(group.id)!.then(async (groupPictureBlobSizeBigInt) => {
                    const profilePictureBlobSize = Number(groupPictureBlobSizeBigInt);
                    const profilePictureData = new Uint8Array(profilePictureBlobSize);

                    const promises = [];
                    for (let i = 0; i < Math.ceil(profilePictureBlobSize / MB); ++i) {
                        promises.push(actor.value?.get_group_profile_picture_chunk_blob(group.id, BigInt(i)).then((chunk) => {
                            profilePictureData.set(chunk, i * MB);
                        }));
                    }

                    await Promise.all(promises);

                    groupList.value[i].profile_picture_blob = profilePictureData;
                });
            }
        }
    }
    async function createGroup({
        name,
        picture,
    }: {
        name: string;
        picture: Uint8Array;
    }) {
        const groupId = await actor.value?.create_group(name)!;

        for (let i = 0; i < Math.ceil(picture.length / MB); ++i) {
            const start = i * MB;
            const end = Math.min(start + MB, picture.length);
            const chunk = picture.slice(start, end);
            await actor.value?.upload_group_profile_picture(groupId, chunk);
        }
    }

    async function getGroup(id: bigint) {
        const response = await actor.value?.get_group(id);
        if (response) {
            currentGroup.value = response[0];
        }

        return response;
    }

    async function addVideo({
        id,
        screen,
        camera,
    }: {
        id: bigint;
        screen: Uint8Array;
        camera: Uint8Array;
    }) {
        const response = await actor.value?.add_video(id, screen, camera);

        return response;
    }

    async function inviteUser(name: string) {
        const response = await actor.value?.invite_user(name);
        return response;
    }

    return {
        currentGroup,
        groupList,
        groupPicture,

        addVideo,
        getAllGroups,
        getGroup,
        createGroup,
        inviteUser,
    };
});
