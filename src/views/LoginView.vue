<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";
import "../login.css";

const router = useRouter();

const username = ref("");
const password = ref("");

const errMsg = ref("");

async function login() {
  try {
    await invoke("login", {
      username: username.value,
      password: password.value,
    });
    router.push("/workspace");
  } catch (e: any) {
    errMsg.value = e.error || "System Crash";
  }
}

async function onNavigateToCreate() {
  router.push("/signup");
}
</script>

<template>
  <div class="container">
    <h1>Robo pass</h1>

    <div class="row">
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
    </div>

    <form class="login-box" onsubmit="event.preventDefault();">
      <input v-model="username" placeholder="Enter an username..." required />
      <input
        v-model="password"
        placeholder="Master password"
        type="password"
        required
      />
      <button type="submit" id="login-btn" @click="login()">Log In</button>
      <button class="white-effect" @click="onNavigateToCreate">
        Create an account
      </button>
      <div v-show="errMsg" class="alert">
        <p>{{ errMsg }}</p>
      </div>
    </form>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.login-box > #login-btn {
  margin: 60px 0 36px 0;
  will-change: filter;
}

.login-box > #login-btn:hover {
  filter: drop-shadow(0 0 1.5em #ffc130);
}

.login-box > #login-btn:focus {
  filter: drop-shadow(0 0 1.5em #ffc130);
}
</style>