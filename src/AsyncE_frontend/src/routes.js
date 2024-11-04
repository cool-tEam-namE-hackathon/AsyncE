import { Home, CreateGroup, GroupPage, GroupList } from "@lazy-loading-routes";
import { createRouter, createWebHistory } from "vue-router";

const routes = [
    {
        path: "/",
        component: Home,
    },
    {
        path: "/create-group",
        component: CreateGroup,
    },
    {
        path: "/group/:id",
        component: GroupPage,
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
