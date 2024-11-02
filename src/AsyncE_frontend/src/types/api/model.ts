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
	selectedCamera?: string;
	cameraList: Option[];
	enabledCamera: boolean;
	isRecordingDisabled: boolean;
	recordingPhaseText: string;
}

interface BaseDialogProps {
	open?: boolean;
	hideCloseButton?: boolean;
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
	placeholder: string;
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
	VideoControlProps,
	BaseProgress,
	RecordedChunks,
	MediaRecorders,
	RecordingStatus,
	Elements,
	VideoRefs,
	GroupInvite,
};
