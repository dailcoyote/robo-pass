<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { reactive, onMounted } from "vue";
import { useRouter } from "vue-router";
import CredentialBox from "../components/CredentialBox.vue";
import AddCredentialModal from "../components/AddCredentialModal.vue";
import { Credential } from "../types";

enum EditionMode {
  Create = 0x01,
  Update = 0x02,
}

const router = useRouter();
const state = reactive({
  addCredentialDialogVisible: false,
  dialog: {
    credentialID: "",
    url: "",
    username: "",
    password: "",
    mode: undefined,
  },
  credentialsSharedMemory: new Map<String, Credential>(),
  lastCredentialModifications: new Set<String>(),
  infoBoard: "",
  validatorBox: "",
});

function openCredentialDialog(uniqid: String | null) {
  state.addCredentialDialogVisible = true;
  state.validatorBox = "";
  if (!uniqid) {
    state.dialog.mode = EditionMode.Create;
  }
  if (typeof uniqid === "string") {
    let { url, username, password } = state.credentialsSharedMemory.get(uniqid);
    state.dialog.credentialID = uniqid;
    state.dialog.url = url;
    state.dialog.username = username;
    state.dialog.password = password;
    state.dialog.mode = EditionMode.Update;
  }
}

function closeCredentialDialog() {
  state.validatorBox = "";
  state.dialog.credentialID = "";
  state.dialog.url = "";
  state.dialog.username = "";
  state.dialog.password = "";
  state.dialog.mode = undefined;
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

async function saveKeeperCredential() {
  let url = state.dialog.url;
  let username = state.dialog.username;
  let password = state.dialog.password;

  try {
    if (state.dialog.mode == EditionMode.Create) {
      state.dialog.credentialID = await invoke("add_privacy", {
        url,
        username,
        password,
      });
    } else if (state.dialog.mode == EditionMode.Update) {
      let uniqid = state.dialog.credentialID;
      await invoke("update_privacy", {
        uniqid,
        url,
        username,
        password,
      });
    } else {
      return;
    }
    state.lastCredentialModifications.add(state.dialog.credentialID);
    fetchKeeperCredentials();
    closeCredentialDialog();
  } catch (e: any) {
    state.validatorBox = e;
  }
}

async function removeKeeperCredential(uniqid: String) {
  try {
    await invoke("remove_privacy", {
      uniqid,
    });
    state.credentialsSharedMemory.delete(uniqid);
  } catch (e: any) {
    state.infoBoard = e.error || "Error removing credentials";
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
        <CredentialBox
          :url="url"
          :username="username"
          :password="password"
          :class="[
            state.lastCredentialModifications.has(key)
              ? 'active-credential__box'
              : '',
          ]"
          @on-edit="
            () => {
              openCredentialDialog(key);
            }
          "
          @on-remove="
            () => {
              removeKeeperCredential(key);
            }
          "
        />
      </template>
    </div>

    <AddCredentialModal v-show="state.addCredentialDialogVisible">
      <template v-slot:body>
        <form
          class="add-credential__form_box"
          @submit.prevent="saveKeeperCredential"
        >
          <input v-model="state.dialog.url" placeholder="Enter a url..." />
          <input
            v-model="state.dialog.username"
            placeholder="Enter an username..."
          />
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
          <div v-show="state.validatorBox" class="alert">
            <p>{{ state.validatorBox }}</p>
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

.active-credential__box {
  border: 1px solid #4fffb0;
}
</style>