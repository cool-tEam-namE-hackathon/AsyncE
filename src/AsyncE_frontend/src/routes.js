import { storeToRefs } from "pinia";
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
import { useUserStore } from "@stores/user-store";
import { useToast } from "@components/ui/toast";

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

// router.beforeEach((to, from) => {
//     const userStore = useUserStore();

//     const { isReady, isAuthenticated } = storeToRefs(userStore);

//     const { toast } = useToast();

//     if (isReady.value && !isAuthenticated.value && to.name !== "Home") {
//         toast({
//             title: "You are not logged in",
//             description: "Please log in to continue",
//             class: "flex flex-col items-start gap-2 p-4 bg-red-100 border-l-4 border-red-500 text-red-700 rounded-md shadow-md",
//         });

//         return { name: "Home" };
//     }
// });

export default router;
