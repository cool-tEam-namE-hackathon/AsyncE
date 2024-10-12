interface User {
    profile_picture_blob: Uint8Array;
    username: [string];
}

interface Group {
    id: bigint;
    profile_picture_blob: Uint8Array | number[];
    owner: string;
    name: string;
    users: string[];
}

interface Video {
    id?: bigint;
    screen: Uint8Array | null;
    camera: Uint8Array | null;
}

interface RecordedChunks {
    screen: Blob[];
    camera: Blob[];
}

interface MediaRecorders {
    screen: MediaRecorder | null;
    camera: MediaRecorder | null;
}

interface BaseDialogProps {
    open?: boolean;
    hideCloseButton?: boolean;
}

interface BaseDropdownProps {
    options?: Option[];
    label?: string;
    side?: "top" | "right" | "bottom" | "left";
}

interface BaseSelectProps {
    modelValue?: string;
    placeholder: string;
    options?: Option[];
}

interface Option {
    deviceId: string;
    name: string;
}

export type {
    User,
    Group,
    Video,
    BaseDialogProps,
    BaseDropdownProps,
    BaseSelectProps,
    RecordedChunks,
    MediaRecorders,
};
