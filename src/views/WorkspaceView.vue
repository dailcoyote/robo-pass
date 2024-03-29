<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, reactive, onMounted } from "vue";
import { useRouter } from "vue-router";
import CredentialBox from "../components/CredentialBox.vue";
import AddCredentialModal from "../components/AddCredentialModal.vue";
import { Credential } from "../types";

const router = useRouter();
const state = reactive({
  addCredentialDialogVisible: false,
  dialog: {
    url: "",
    username: "",
    password: "",
  },
  credentialsSharedMemory: new Map<String, Credential>(),
  lastCredentialModifications: new Array<String>(),
  infoBoard: "",
  validatorBox: "",
});

function openCredentialDialog() {
  state.addCredentialDialogVisible = true;
}

function closeCredentialDialog() {
  state.validatorBox = "";
  state.dialog.url = "";
  state.dialog.username = "";
  state.dialog.password = "";
  state.addCredentialDialogVisible = false;
}

async function fetchKeeperCredentials() {
  try {
    state.credentialsSharedMemory.clear();
    const heap: Object = await invoke("fetch_privacy_heap");
    state.credentialsSharedMemory = new Map<String, Credential>(
      Object.entries(heap)
    );
  } catch (e: any) {
    state.infoBoard = e.error || "Error reading credentials";
  }
}

async function addKeeperCredential() {
  let url = state.dialog.url;
  let username = state.dialog.username;
  let password = state.dialog.password;

  try {
    state.validatorBox = "";
    const uuid: String = await invoke("add_privacy", {
      url,
      username,
      password,
    });
    state.lastCredentialModifications.push(uuid);
    fetchKeeperCredentials();
    closeCredentialDialog();
  } catch (e: any) {
    state.validatorBox = e.error || "Error inserting credentials";
  }
}

function logout() {
  invoke("logout");
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

    {{ state.infoBoard }}

    <div class="credential--box-list">
      <template
        v-for="[
          key,
          { url, username, password },
        ] in state.credentialsSharedMemory"
        :key="key"
      >
        <CredentialBox :url="url" :username="username" :password="password" 
        :class="[state.lastCredentialModifications.includes(key) ? 'active-credential__box' : '']" />
      </template>
    </div>

    <AddCredentialModal v-show="state.addCredentialDialogVisible">
      <template v-slot:body>
        <form
          class="add-credential__form_box"
          @submit.prevent="addKeeperCredential"
        >
          <input v-model="state.dialog.url" placeholder="Enter a url..." />
          <input v-model="state.dialog.username" placeholder="Enter an username..." />
          <input
            v-model="state.dialog.password"
            placeholder="Enter a password..."
            type="password"
          />
          <div class="row">
            <button class="bluesky-effect" type="submit">Create</button>
            <button class="vermillion-effect" @click="closeCredentialDialog()">
              Cancel
            </button>
          </div>
          <p class="alert" v-show="state.validatorBox">
            {{ state.validatorBox }}
          </p>
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

.active-credential__box {
  border: 1px solid #4FFFB0;
}
</style>