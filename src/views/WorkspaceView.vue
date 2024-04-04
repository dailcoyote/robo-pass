<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { reactive, onMounted } from "vue";
import { useRouter } from "vue-router";
import CredentialBox from "../components/CredentialBox.vue";
import AddCredentialModal from "../components/AddCredentialModal.vue";
import { Credential, KeeperCredential } from "../types";

enum EditionMode {
  Non = 0x00,
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
    mode: EditionMode.Non,
  },
  keeperCredentialsSharedVector: new Array<KeeperCredential>(),
  lastCredentialModifications: new Set<string>(),
  infoBox: "",
  validatorBox: "",
});

function openCredentialDialog(uniqid: string | null) {
  state.validatorBox = "";
  if (!uniqid) {
    state.dialog.mode = EditionMode.Create;
  }
  if (typeof uniqid === "string") {
    let credential: Credential | undefined =
      state.keeperCredentialsSharedVector.find(
        (item: KeeperCredential) => item.keeper_id == uniqid
      )?.privacy || undefined;

    if (!credential) {
      state.infoBox = "Credential not found";
      return;
    }

    state.dialog.credentialID = uniqid;
    state.dialog.url = credential.url;
    state.dialog.username = credential.username;
    state.dialog.password = credential.password;
    state.dialog.mode = EditionMode.Update;
  }

  state.addCredentialDialogVisible = true;
}

function closeCredentialDialog() {
  state.dialog.credentialID = "";
  state.dialog.url = "";
  state.dialog.username = "";
  state.dialog.password = "";
  state.dialog.mode = EditionMode.Non;
  state.addCredentialDialogVisible = false;
}

async function fetchSortedKeeperCredentials() {
  try {
    const heap: Array<KeeperCredential> = await invoke(
      "fetch_sorted_privacy_vec"
    );
    state.keeperCredentialsSharedVector = [...heap];
  } catch (e: any) {
    state.infoBox = e.error || "Error reading credentials";
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
      throw new Error("EditionMode Not Found");
    }
    state.lastCredentialModifications.add(state.dialog.credentialID);
    fetchSortedKeeperCredentials();
    closeCredentialDialog();
  } catch (e: any) {
    state.validatorBox = e;
  }
}

async function removeKeeperCredential(uniqid: string) {
  try {
    await invoke("remove_privacy", {
      uniqid,
    });
    fetchSortedKeeperCredentials();
  } catch (e: any) {
    state.infoBox = e.error || "Error removing credentials";
  }
}

function logout() {
  state.lastCredentialModifications.clear();
  state.keeperCredentialsSharedVector = [];
  invoke("logout");
  router.push("/");
}

onMounted(() => {
  fetchSortedKeeperCredentials();
});
</script>

<template>
  <div class="container">
    <div id="credential--toolbar">
      <button id="add_credential--button" @click="openCredentialDialog(null)">
        Add Credentials
      </button>
      <button id="logout--button" @click="logout()">Logout</button>
    </div>

    <div class="infoBox">
      <p v-show="state.infoBox">{{ state.infoBox }}</p>
    </div>

    <div class="credential--box-list">
      <template
        v-for="{ keeper_id, privacy } in state.keeperCredentialsSharedVector"
        :key="keeper_id"
      >
        <CredentialBox
          :url="privacy.url"
          :username="privacy.username"
          :password="privacy.password"
          :class="[
            state.lastCredentialModifications.has(keeper_id)
              ? 'active-credential__box'
              : '',
          ]"
          @on-edit="
            () => {
              openCredentialDialog(keeper_id);
            }
          "
          @on-remove="
            () => {
              removeKeeperCredential(keeper_id);
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
          <div v-show="state.validatorBox" class="alertBox">
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