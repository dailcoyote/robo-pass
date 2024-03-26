<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";
import "../login.css";

const router = useRouter();

const name = ref("");
const password = ref("");
const retypePassword = ref("");

const errMsg = ref("");
const infoMsg = ref("");

function back() {
  router.push("/");
}

async function create() {
  try {
    await invoke("create_account", {
      username: name.value,
      password: password.value
    });
    router.push("/workspace");
  } catch (error: any) {
    errMsg.value = error || "Crash";
  }
}
</script>

<template>
  <div class="container">
    <h1>Create an account</h1>

    <div class="start_row">
      <button class="lemon-effect" @click="back">Back</button>
    </div>

    {{infoMsg}}

    <form class="login-box" onsubmit="event.preventDefault();">
      <input v-model="name" placeholder="Enter an username..." required />
      <input
        v-model="password"
        placeholder="Master password"
        type="password"
        required
      />
      <input
        v-model="retypePassword"
        placeholder="Retype password"
        type="password"
        required
      />
      <button type="submit" id="create-btn" @click="create()">Create</button>
      {{errMsg}}
    </form>
  </div>
</template>

<style scoped>
.login-box > #create-btn {
  margin: 60px 0;
  will-change: filter;
}

.login-box > #create-btn:hover {
  filter: drop-shadow(0 0 1.5em #68be85);
}

.login-box > #create-btn:focus {
  filter: drop-shadow(0 0 1.5em #68be85);
}
</style>