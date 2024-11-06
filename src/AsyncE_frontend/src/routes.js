import { createRouter, createWebHistory } from "vue-router";

import {
    Home,
    CreateGroup,
    GroupPage,
    GroupList,
    MeetingPage,
    NotFoundPage,
    ProfilePage,
} from "@lazy-loading-routes";

const routes = [
    {
        path: "/",
        name: "Home",
        component: Home,
    },
    {
        path: "/profile",
        name: "ProfilePage",
        component: ProfilePage,
    },
    {
        path: "/create-group",
        name: "CreateGroup",
        component: CreateGroup,
    },
    {
        path: "/group-list",
        name: "GroupList",
        component: GroupList,
    },
    {
        path: "/group/:id",
        name: "GroupPage",
        component: GroupPage,
    },
    {
        path: "/group/:groupId/meeting/:meetingId",
        name: "MeetingPage",
        component: MeetingPage,
    },
    {
        path: "/:pathMatch(.*)*",
        name: "NotFound",
        component: NotFoundPage,
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
