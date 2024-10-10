import { createRouter, createWebHistory } from "vue-router";

import Home from "./pages/Home.vue";
import Group from "./pages/Group.vue";
import Video from "./pages/Video.vue";
import GroupList from "./pages/GroupList.vue";

const routes = [
    {
        path: "/",
        component: Home,
    },
    {
        path: "/group",
        component: Group,
    },
    {
        path: "/create-video",
        component: Video,
    },
    {
        path: "/group-list",
        component: GroupList,
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
