import { createApp } from "vue";
import { createMemoryHistory, createRouter } from 'vue-router'
import "./styles.css";
import App from "./App.vue";


import LoginView from './views/LoginView.vue'
import SignUpView from "./views/SignUpView.vue";
import WorkspaceView from './views/WorkspaceView.vue'

const routes = [
    { path: '/', name: "login", component: LoginView },
    { path: '/signup', name: "signup",  component: SignUpView },
    { path: '/workspace', name: "workspace",  component: WorkspaceView },
]

const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

const app = createApp(App);
app.use(router);
app.mount("#app");
