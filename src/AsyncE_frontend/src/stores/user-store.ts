import { ref, toRaw } from "vue";
import { defineStore } from "pinia";

import { createActor } from "@declarations/AsyncE_backend/index";

import { AuthClient } from "@dfinity/auth-client";
import { Identity } from "@dfinity/agent";
import type { _SERVICE } from "../../../declarations/AsyncE_backend/AsyncE_backend.did";
import { User } from "@/types/api/model";

const client = ref<AuthClient | null>(null);
const isAuthenticated = ref<boolean>(false);
const identity = ref<Identity | null>(null);
const actor = ref<_SERVICE | null>();

const username = ref<string | null>();

export const getIdentityProvider = () => {
    let idpProvider;
    // Safeguard against server rendering
    if (typeof window !== "undefined") {
        const isLocal = process.env.DFX_NETWORK !== "ic";
        // Safari does not support localhost subdomains
        const isSafari = /^((?!chrome|android).)*safari/i.test(
            navigator.userAgent,
        );
        if (isLocal && isSafari) {
            idpProvider = `http://localhost:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`;
        } else if (isLocal) {
            idpProvider = `http://${process.env.CANISTER_ID_INTERNET_IDENTITY}.localhost:4943`;
        }
    }
    return idpProvider;
};

export const defaultOptions = {
    createOptions: {
        idleOptions: {
            disableIdle: true,
        },
    },
    loginOptions: {
        identityProvider: getIdentityProvider(),
    },
};

function actorFromIdentity(identity: Identity) {
    return createActor(process.env.CANISTER_ID_ASYNCE_BACKEND, {
        agentOptions: {
            identity,
        },
    });
}

export const useUserStore = defineStore("user", () => {
    async function init() {
        client.value = await AuthClient.create(defaultOptions.createOptions);
        isAuthenticated.value = await client.value.isAuthenticated();
        identity.value = isAuthenticated.value
            ? client.value.getIdentity()
            : null;
        actor.value = identity.value ? actorFromIdentity(identity.value) : null;
    }

    async function login() {
        const authClient = toRaw(client.value);

        await new Promise<void>((resolve) => {
            authClient?.login({
                ...defaultOptions.loginOptions,
                identityProvider: getIdentityProvider(),
                async onSuccess() {
                    isAuthenticated.value = await authClient.isAuthenticated();
                    identity.value = isAuthenticated.value
                        ? authClient.getIdentity()
                        : null;
                    actor.value = identity.value
                        ? actorFromIdentity(identity.value)
                        : null;
                    resolve();
                },
            });
        });
    }

    async function logout() {
        const authClient = toRaw(client.value);
        await authClient?.logout();
        isAuthenticated.value = false;
        identity.value = null;
        actor.value = null;
    }

    async function register(payload: User) {
        const response = await actor.value?.register(payload);

        return response;
    }

    async function getUserCredentials() {
        const response = await actor.value?.get_user_credentials();
        if (response) {
            username.value = response[0]?.username[0];
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

    return {
        isAuthenticated,
        identity,
        actor,
        username,

        init,
        login,
        logout,
        register,
        createGroup,
        getUserCredentials,
    };
});
