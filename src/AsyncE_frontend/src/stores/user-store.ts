import { ref, toRaw } from "vue";

import { defineStore } from "pinia";

import {
    _SERVICE,
    UserCredentialsResponse,
} from "@declarations/AsyncE_backend/AsyncE_backend.did";
import { createActor } from "@declarations/AsyncE_backend/index";
import { ActorSubclass, Identity } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";

import { MB } from "@/data/user-constants";
import { blobToURL, validateResponse } from "@/utils/helpers";

const client = ref<AuthClient | null>(null);
const isAuthenticated = ref<boolean>(false);
const identity = ref<Identity | null>(null);
const actor = ref<ActorSubclass<_SERVICE> | null>();

const userCredentials = ref<UserCredentialsResponse>();
const profilePicture = ref<string>("");

export const getIdentityProvider = () => {
    let idpProvider;

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
    return createActor(process.env.CANISTER_ID_ASYNCE_BACKEND || "", {
        agentOptions: {
            identity,
        },
    });
}

export const useUserStore = defineStore("user", () => {
    async function init() {
        client.value = await AuthClient.create(defaultOptions.createOptions);
        isAuthenticated.value = await client.value.isAuthenticated();
        if (isAuthenticated.value) {
            identity.value = client.value.getIdentity();
            actor.value = actorFromIdentity(identity.value);
        } else {
            identity.value = null;
            actor.value = null;
        }
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

    async function register({
        username,
        profile_picture_blob,
    }: {
        username: string;
        profile_picture_blob: Uint8Array;
    }) {
        const response = await actor.value?.register(username);

        validateResponse(response);

        for (let i = 0; i < Math.ceil(profile_picture_blob.length / MB); ++i) {
            const start = i * MB;
            const end = Math.min(start + MB, profile_picture_blob.length);
            const chunk = profile_picture_blob.slice(start, end);
            await actor.value?.upload_profile_picture(
                chunk,
                BigInt(i),
                BigInt(profile_picture_blob.length),
            );
        }
    }

    async function getUserCredentials() {
        const response = await actor.value?.get_user_credentials();

        const okResponse = validateResponse(response);

        if (!okResponse[0]) return;

        userCredentials.value = okResponse[0];

        const profilePictureSize =
            await actor.value?.get_profile_picture_size();

        const profilePictureBlobSizeBigint =
            validateResponse(profilePictureSize);

        const profilePictureBlobSize = Number(profilePictureBlobSizeBigint);
        const profilePictureData = new Uint8Array(profilePictureBlobSize);

        for (let i = 0; i < Math.ceil(profilePictureBlobSize / MB); ++i) {
            await actor.value
                ?.get_profile_picture_chunk_blob(BigInt(i))
                .then((chunk) => {
                    const okChunk = validateResponse(chunk);
                    profilePictureData.set(okChunk, i * MB);
                });
        }

        profilePicture.value = blobToURL(profilePictureData);

        return okResponse;
    }

    async function validateUsername(username: string) {
        const response = await actor.value?.validate_username(username);

        const okResponse = validateResponse(response);

        return okResponse;
    }

    return {
        isAuthenticated,
        identity,
        actor,
        userCredentials,
        profilePicture,

        init,
        login,
        logout,
        register,
        validateUsername,
        getUserCredentials,
    };
});
