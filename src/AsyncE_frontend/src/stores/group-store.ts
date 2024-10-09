import { defineStore } from "pinia";

import { AsyncE_backend } from "@declarations/AsyncE_backend/index";

export const useGroupStore = defineStore("group", () => {
    async function createGroup({
        name,
        picture,
    }: {
        name: string;
        picture: Uint8Array;
    }) {
        const response = await AsyncE_backend.create_group(name, picture);

        return response;
    }

    return {
        createGroup,
    };
});
