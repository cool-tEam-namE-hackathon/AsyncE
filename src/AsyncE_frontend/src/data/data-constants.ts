const MB = Math.floor(1024.0 * 1024.0 * 1.8);
const USER_DROPDOWN_OPTIONS = [
    {
        name: "Profile",
    },
    {
        name: "Logout",
        class: "text-red-500",
    },
];

const CHAT_OPTIONS = [
    {
        name: "Edit chat",
    },
    {
        name: "Delete chat",
        class: "text-red-500",
    },
];

export { USER_DROPDOWN_OPTIONS, CHAT_OPTIONS, MB };
