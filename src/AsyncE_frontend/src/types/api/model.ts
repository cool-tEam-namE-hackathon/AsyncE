import {
    GroupMemberRole,
    MeetingHeader,
    MeetingProcessType,
} from "@declarations/AsyncE_backend/AsyncE_backend.did";

interface User {
    profile_picture_blob: Uint8Array;
    username: string;
}

interface GroupMember {
    role: GroupMemberRole;
    username: string;
}

interface MeetingList {
    id: bigint;
    title: string;
    created_time_unix: bigint;
    process_type: MeetingProcessType;
    created_by: string;
    frames_count: bigint;
    thumbnail_blob?: Uint8Array | number[];
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
    class?: string;
    hasSeparator?: boolean;
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

interface Message {
    id: bigint;
    content: string;
    username: string;
    created_time_unix: bigint;
    uuid: string;
    group_id: bigint;
}

interface EditChat {
    group_id: bigint;
    new_content: string;
    chat_id: bigint;
}

interface DeleteChat {
    chat_id: bigint;
    group_id: bigint;
}

type RoleKeys = "Admin" | "Member";

export type {
    RoleKeys,
    User,
    Message,
    Group,
    Video,
    DeleteChat,
    BaseDialogProps,
    BaseDropdownProps,
    BaseSelectProps,
    VideoList,
    VideoControlProps,
    DropdownOptions,
    BaseProgress,
    RecordedChunks,
    MediaRecorders,
    RecordingStatus,
    Elements,
    MeetingList,
    EditChat,
    VideoRefs,
    GroupInvite,
};
