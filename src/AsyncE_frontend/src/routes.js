import {
    Home,
    CreateGroup,
    GroupPage,
    GroupList,
    MeetingPage,
    NotFoundPage,
} from "@lazy-loading-routes";
import { createRouter, createWebHistory } from "vue-router";

const routes = [
    {
        path: "/",
        name: "Home",
        component: Home,
    },
    {
        path: "/create-group",
        name: "CreateGroup",
        component: CreateGroup,
    },
    {
        path: "/group/:id",
        name: "GroupPage",
        component: GroupPage,
    },
    {
        path: "/group-list",
        name: "GroupList",
        component: GroupList,
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
