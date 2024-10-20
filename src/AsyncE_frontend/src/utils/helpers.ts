import { RecordedChunks } from "@/types/api/model";

function fileToBlob(file: File) {
    return file
        ? new Blob([file], {
              type: file.type,
          })
        : null;
}

function blobToURL(data: Uint8Array | number[]) {
    const temp = new Uint8Array(data);

    const blob = new Blob([temp], {
        type: "image/jpeg",
    });

    return URL.createObjectURL(blob);
}

function createChunks(array: Uint8Array, size: number) {
    const chunks = [];

    for (let i = 0; i < array.length; i += size) {
        chunks.push(array.subarray(i, i + size));
    }

    return chunks;
}

function generateUUID() {
    return crypto.randomUUID();
}

function validateResponse<T>(
    response: { Ok: T } | { Err: string } | undefined | null,
): T {
    if (!response) {
        throw new Error("Response is null or undefined");
    }

    if (!("Ok" in response)) {
        throw Error(response.Err);
    }

    return response.Ok;
}

function timeAgo(timestamp: bigint) {
    const timestampMillis = Number(timestamp / 1000000n); // Accurate conversion
    const date = new Date(timestampMillis);

    console.log(date);

    const now = new Date().getTime();
    const diff = now - date.getTime();

    const intervals = [
        { label: "year", seconds: 31536000 },
        { label: "month", seconds: 2628000 },
        { label: "day", seconds: 86400 },
        { label: "hour", seconds: 3600 },
        { label: "minute", seconds: 60 },
        { label: "second", seconds: 1 },
    ];

    for (const interval of intervals) {
        const milliseconds = interval.seconds * 1000;
        const count = Math.floor(diff / milliseconds);

        if (count > 0) {
            return `${count} ${interval.label}${count > 1 ? "s" : ""} ago`;
        }
    }

    return "just now";
}

export {
    fileToBlob,
    blobToURL,
    createChunks,
    generateUUID,
    validateResponse,
    timeAgo,
};
