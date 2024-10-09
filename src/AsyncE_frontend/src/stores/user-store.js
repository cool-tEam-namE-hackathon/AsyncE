import { defineStore } from "pinia";

import {
    createActor,
    AsyncE_backend,
} from "@declarations/AsyncE_backend/index";

import { AuthClient } from "@dfinity/auth-client";
import { HttpAgent } from "@dfinity/agent";

export const useUserStore = defineStore("user", () => {
    let be = AsyncE_backend;
    async function login() {
        let authClient = await AuthClient.create();

        const network =
            process.env.DFX_NETWORK ||
            (process.env.NODE_ENV === "production" ? "ic" : "local");
        const internetIdentityUrl =
            network === "local"
                ? `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943/`
                : `https://identity.ic0.app`;

        await new Promise((resolve) => {
            authClient.login({
                identityProvider: internetIdentityUrl,
                onSuccess: resolve,
            });
        });

        const identity = authClient.getIdentity();
        const agent = HttpAgent.createSync({ identity });

        const canisterId = process.env.CANISTER_ID_ASYNCE_BACKEND || "";
        be = createActor(canisterId, {
            agent,
        });

        const response = await be.login();

        return response;
    }

    async function register(payload) {
        const response = await be.register(payload);

        return response;
    }

    async function getAllUsernames() {
        const response = await be.get_all_usernames();

        return response;
    }

    return {
        register,
        login,
        getAllUsernames,
    };
});
