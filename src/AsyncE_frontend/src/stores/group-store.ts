import { ref } from "vue";

import { defineStore } from "pinia";

import { useUserStore } from "@stores/user-store";

import { storeToRefs } from "pinia";

import { Group } from "@/types/api/model";

export const useGroupStore = defineStore("group", () => {
    const { actor } = storeToRefs(useUserStore());

    const groupPicture = ref<string>("");
    const groupList = ref<Group[]>([]);
    const currentGroup = ref<Group>();

    async function getAllGroups() {
        const response = await actor.value?.get_all_groups();
        if (response) {
            groupList.value = response;
        }
    }
    async function createGroup({
        name,
        picture,
    }: {
        name: string;
        picture: Uint8Array;
    }) {
        const response = await actor.value?.create_group(name, picture);

        return response;
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

    return {
        currentGroup,
        groupList,
        groupPicture,

        addVideo,
        getAllGroups,
        getGroup,
        createGroup,
    };
});
