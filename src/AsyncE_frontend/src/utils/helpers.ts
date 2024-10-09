function fileToBlob(file: File) {
    return file
        ? new Blob([file], {
              type: file.type,
          })
        : null;
}

export { fileToBlob };
