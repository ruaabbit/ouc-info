import { createRouter, createWebHashHistory } from "vue-router";
import Login from "../components/Login.vue";
import Main from "../components/Main.vue";

const routes = [
    { path: '/', component: Login },
    { path: '/main', component: Main },
]

const router = createRouter({
    history: createWebHashHistory(),
    routes,
})

export default router