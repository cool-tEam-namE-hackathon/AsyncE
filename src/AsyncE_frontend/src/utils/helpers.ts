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

function convertDateToReadableFormat(timestamp: bigint) {
    const dateInMilliseconds = Number(timestamp) / 1e6;
    return new Date(dateInMilliseconds).toLocaleDateString("en-US", {
        day: "numeric",
        month: "short",
        year: "numeric",
    });
}

export {
    fileToBlob,
    blobToURL,
    generateUUID,
    validateResponse,
    convertDateToReadableFormat,
};
