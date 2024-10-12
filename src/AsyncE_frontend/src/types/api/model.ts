interface User {
    profile_picture_blob: Uint8Array;
    username: string;
}

interface Group {
    id: bigint;
    profile_picture_blob: Uint8Array | number[];
    owner: string;
    name: string;
    users: string[];
}

interface Video {
    id: bigint;
    webcam_blob: Blob;
    screen_blob: Blob;
}

interface BaseDialogProps {
    open?: boolean;
    hideCloseButton?: boolean;
}

interface BaseDropdownProps {
    options?: UserDropdownOption[];
    label?: string;
}

interface UserDropdownOption {
    name: string;
}

export type { User, Group, Video, BaseDialogProps, BaseDropdownProps };
