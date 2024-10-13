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
export { fileToBlob, blobToURL, createChunks, generateUUID };
