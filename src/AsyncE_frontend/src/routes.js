import { createRouter, createWebHistory } from "vue-router";

import Home from "./pages/Home.vue";
import CreateGroup from "./pages/CreateGroup.vue";
import GroupPage from "./pages/GroupPage.vue";
import GroupList from "./pages/GroupList.vue";

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
