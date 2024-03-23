<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import "../login.css";

const router = useRouter();

const name = ref("");
const password = ref("");

async function login() {
  router.push("/workspace");
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

    <form class="login-box">
      <input v-model="name" placeholder="Enter an username..." required/>
      <input v-model="password" placeholder="Master password" type="password" required/>
      <button type="submit" id="login-btn" @click="login()">Log In</button>
      <button class="white-effect" @click="onNavigateToCreate">Create an account</button>
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