<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();

const name = ref("");
const password = ref("");

async function login() {
  console.log("login...");
  router.push("/workspace");
}
</script>

<template>
  <form class="login-box">
    <input v-model="name" placeholder="Enter an username..." />
    <input
      v-model="password"
      placeholder="Enter a password..."
      type="password"
    />
    <button type="submit" @click="login()">Log In</button>
  </form>
</template>

<style scoped>
.login-box {
  display: flex;
  margin: 0 auto;
  flex-direction: column;
  justify-content: center;
  padding: 5%;
  width: 25%;
  /* opacity: 0.35; */
  shape-image-threshold: 70%;
  outline: none;
}

.login-box > input {
  margin: 24px 0;
  will-change: filter;
}

.login-box > input:hover {
  filter: drop-shadow(0 0 1em #24c8db);
}

.login-box > input:focus {
  filter: drop-shadow(0 0 1em #24c8db);
}

.login-box > button {
  margin: 36px 0;
  will-change: filter;
}

.login-box > button:hover {
  filter: drop-shadow(0 0 1.5em #ffc130);
}

.login-box > button:focus {
  filter: drop-shadow(0 0 1.5em #ffc130);
}
</style>
