<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import "../login.css";

const router = useRouter();

const name = ref("");
const password = ref("");
const retypePassword = ref("");

function back() {
  router.push("/");
}

async function create() {
  router.push("/workspace");
}
</script>

<template>
  <div class="container">
    <h1>Create an account</h1>

    <div class="start_row">
      <button class="lemon-effect" @click="back">Back</button>
    </div>

    <form class="login-box">
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