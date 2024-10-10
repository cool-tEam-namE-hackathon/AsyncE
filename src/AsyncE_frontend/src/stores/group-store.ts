import { ref } from "vue";

import { defineStore } from "pinia";

import { useUserStore } from "@stores/user-store";

import { storeToRefs } from "pinia";

import { blobToURL } from "@/utils/helpers";

export const useGroupStore = defineStore("group", () => {
    const { actor } = storeToRefs(useUserStore());

    const groupPicture = ref<string>("null");

    async function getAllGroups() {
        const response = await actor.value?.get_all_groups();
        console.log("asd", response);
        if (response) {
            if (response[0].profile_picture_blob) {
                groupPicture.value = blobToURL(
                    response[0]?.profile_picture_blob,
                );
            }
        }

        return response;
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

    // async function addVideo() {
    //     const response = await actor.value?.add_video();

    //     return response;
    // }

    return {
        groupPicture,

        getAllGroups,
        createGroup,
    };
});
