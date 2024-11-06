import {
    GroupMemberRole,
    MeetingHeader,
} from "@declarations/AsyncE_backend/AsyncE_backend.did";

interface User {
    profile_picture_blob: Uint8Array;
    username: string;
}

interface GroupMember {
    role: GroupMemberRole;
    username: string;
}

interface Group {
    id: bigint;
    profile_picture_blob: Uint8Array | number[];
    owner: string;
    name: string;
    members: GroupMember[];
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

interface RecordingStatus {
    screen: boolean;
    camera: boolean;
}

interface Elements {
    container: HTMLElement | null;
    video: HTMLElement | null;
}

interface VideoRefs {
    screen: HTMLVideoElement | null;
    camera: HTMLVideoElement | null;
}

interface VideoControlProps {
    modelValue: string;
    cameraList: Option[];
    enabledCamera: boolean;
    enabledScreen?: boolean;
    isRecordingDisabled: boolean;
    isControlDisabled: boolean;
    recordingPhaseText: string;
}

interface VideoList {
    video: MeetingHeader;
    url: string;
}

interface BaseDialogProps {
    open?: boolean;
    hideCloseButton?: boolean;
    isClosable: boolean;
}

interface DropdownOptions {
    name: string;
}

interface BaseDropdownProps {
    options: DropdownOptions[];
    emptyMessage?: string;
    label?: string;
    side?: "top" | "right" | "bottom" | "left";
}

interface BaseSelectProps {
    modelValue?: string;
    placeholder?: string;
    options?: Option[];
}

interface BaseProgress {
    modelValue: number;
}

interface Option {
    deviceId: string;
    name: string;
}

interface GroupInvite {
    group_id: bigint;
    group_name: string;
}

export type {
    User,
    Group,
    Video,
    BaseDialogProps,
    BaseDropdownProps,
    BaseSelectProps,
    VideoList,
    VideoControlProps,
    BaseProgress,
    RecordedChunks,
    MediaRecorders,
    RecordingStatus,
    Elements,
    VideoRefs,
    GroupInvite,
};
