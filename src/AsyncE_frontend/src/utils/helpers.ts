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
export { fileToBlob, blobToURL };
