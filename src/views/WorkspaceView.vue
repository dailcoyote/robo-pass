<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import CredentialBox from "../components/CredentialBox.vue";
import { useRouter } from "vue-router";

const router = useRouter();

function logout() {
  router.push("/");
}
</script>

<template>
  <div class="container" style="padding: 1% 2%">
    <div id="credential--toolbar">
      <button id="add_credential--button">Add Credentials</button>
      <button id="logout--button" @click="logout()">Logout</button>
    </div>

    <div class="credential--box-list">
      <CredentialBox />
      <CredentialBox />
      <CredentialBox />
      <CredentialBox />
      <CredentialBox />
      <CredentialBox />
      <CredentialBox />
    </div>
  </div>
</template>

<style scoped>
#credential--toolbar {
  display: flex;
  justify-content: space-between;
}
#add_credential--button {
  will-change: filter;
  transition: 0.5s;
}
#add_credential--button:hover {
  filter: drop-shadow(0 0 1em #ffc130);
}
#logout--button {
  will-change: filter;
  transition: 0.5s;
}
#logout--button:hover {
  filter: drop-shadow(0 0 1em #24c8db);
}
.credential--box-list {
  display: flex;
  flex-wrap: wrap;
  margin: 2% 0;
  justify-content: start;
  width: 100%;
  background-color: transparent;
  shape-image-threshold: 70%;
}
</style>