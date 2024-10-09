interface User {
	username: string;
	blob: Blob;
}

interface Group {
	id: string;
	name: string;
	users: string[];
	owner: string;
	profile_picture_blob: Blob;
}

interface Video {
	id: string;
	webcam_blob: Blob;
	screen_blob: Blob;
}

export type { User, Group, Video };
