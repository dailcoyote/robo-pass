<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/tauri";
import "../login.css";

const router = useRouter();

const username = ref("");
const password = ref("");
const retypePassword = ref("");

const errMsg = ref("");

function back() {
  router.push("/");
}

async function create() {
  try {
    if (!username.value) {
      errMsg.value = "Username is empty";
      return;
    }
    if (password.value !== retypePassword.value) {
      errMsg.value = "Password mismatch";
      return;
    }

    await invoke("create_account", {
      username: username.value,
      password: password.value,
    });
    router.push("/workspace");
  } catch (e: any) {
    errMsg.value = e.error || "System Crash";
  }
}
</script>

<template>
  <div class="container">
    <h1>Create an account</h1>

    <div class="start_row">
      <button class="lemon-effect" @click="back">Back</button>
    </div>

    <form class="login-box" onsubmit="event.preventDefault();" autocomplete="off">
      <input v-model="username" placeholder="Enter an username..." required />
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
      <div v-show="errMsg" class="alertBox">
        <p>{{ errMsg }}</p>
      </div>
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