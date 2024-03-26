<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, reactive, onMounted } from "vue";
import { useRouter } from "vue-router";
import CredentialBox from "../components/CredentialBox.vue";
import AddCredentialModal from "../components/AddCredentialModal.vue";
// import CredentialStorage from "../classes/CredentialStorage";
import { Credential } from "../types";

// const credentials = new CredentialStorage();
const router = useRouter();
const state = reactive({
  addCredentialDialogVisible: false,
  url: "",
  username: "",
  password: "",
  credentialsSharedMemory: new Map<String, Credential>(),
});
const infoMsg = ref("");

function clearCredentialDialog() {
  state.url = "";
  state.username = "";
  state.password = "";
}

function openCredentialDialog() {
  state.addCredentialDialogVisible = true;
}

function closeCredentialDialog() {
  clearCredentialDialog();
  state.addCredentialDialogVisible = false;
}

function fetchKeeperCredentials() {
  state.credentialsSharedMemory.clear();
  invoke("fetch_privacy_heap")
    .then((heap: any) => {
      state.credentialsSharedMemory = new Map<String, Credential>(Object.entries(heap));
    })
    .catch((e: any) => {
      infoMsg.value = e;
    });
}

async function addKeeperCredential() {
  let url = state.url;
  let username = state.username;
  let password = state.password;

  try {
    await invoke("add_privacy", {
      url,
      username,
      password,
    });
    fetchKeeperCredentials();
  } catch (e: any) {
    infoMsg.value = e;
  }

  closeCredentialDialog();
}

function logout() {
  router.push("/");
}

onMounted(() => {
  fetchKeeperCredentials();
});
</script>

<template>
  <div class="container">
    <div id="credential--toolbar">
      <button id="add_credential--button" @click="openCredentialDialog()">
        Add Credentials
      </button>
      <button id="logout--button" @click="logout()">Logout</button>
    </div>

    {{ infoMsg }}

    <div class="credential--box-list">
      <template
        v-for="[key, { url, username, password }] in state.credentialsSharedMemory"
        :key="key"
      >
        <CredentialBox :url="url" :username="username" :password="password" />
      </template>
    </div>

    <AddCredentialModal v-show="state.addCredentialDialogVisible">
      <template v-slot:body>
        <form
          class="add-credential__form_box"
          @submit.prevent="addKeeperCredential"
        >
          <input v-model="state.url" placeholder="Enter a url..." />
          <input v-model="state.username" placeholder="Enter an username..." />
          <input
            v-model="state.password"
            placeholder="Enter a password..."
            type="password"
          />
          <div class="row">
            <button class="bluesky-effect" type="submit">Create</button>
            <button class="vermillion-effect" @click="closeCredentialDialog()">
              Cancel
            </button>
          </div>
        </form>
      </template>
    </AddCredentialModal>
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

.add-credential__form_box {
  display: flex;
  margin: 0 auto;
  flex-direction: column;
  justify-content: center;
  padding: 8%;
  /* opacity: 0.35; */
  shape-image-threshold: 70%;
  outline: none;
}

.add-credential__form_box > input {
  margin: 24px 0;
  will-change: filter;
}

.add-credential__form_box > input:hover {
  filter: drop-shadow(0 0 1em #24c8db);
}

.add-credential__form_box > input:focus {
  filter: drop-shadow(0 0 1em #24c8db);
}

.add-credential__form_box > .row > button {
  margin: 36px 18px;
}
</style>