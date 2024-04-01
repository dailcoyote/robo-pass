import { createApp } from "vue";
import { createMemoryHistory, createRouter } from 'vue-router'
import { invoke } from "@tauri-apps/api/tauri";
import "./styles.css";
import App from "./App.vue";

import LoginView from './views/LoginView.vue'
import SignUpView from "./views/SignUpView.vue";
import WorkspaceView from './views/WorkspaceView.vue'


async function canUserAccess(): Promise<boolean> {
    return await invoke("can_user_access");
}

const routes = [
    { path: '/', name: "login", component: LoginView },
    { path: '/signup', name: "signup", component: SignUpView },
    { path: '/workspace', name: "workspace", component: WorkspaceView },
]

const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

router.beforeEach(async (to:any) => {
    if (
        // make sure the user is authenticated
        !await canUserAccess() &&
        // ❗️ Avoid an infinite redirect
        to.name === 'workspace'
    ) {
        // redirect the user to the login page
        return { name: 'login' }
    }
})

const app = createApp(App);
app.use(router);
app.mount("#app");
