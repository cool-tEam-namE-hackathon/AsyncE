import { createRouter, createWebHistory } from "vue-router";

import Home from "./pages/Home.vue";
import Group from "./pages/Group.vue";

const routes = [
    {
        path: "/",
        component: Home,
    },
    {
        path: "/group",
        component: Group,
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
