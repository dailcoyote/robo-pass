<script setup lang="ts">
import { reactive, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "vue-router";
import Snackbar from "awesome-snackbar";
import CredentialBox from "../components/CredentialBox.vue";
import AddCredentialModal from "../components/AddCredentialModal.vue";
import { Credential, KeeperCredential } from "../types";

enum TypeMessages {
  Warning = "w",
  Error = "e",
  Success = "s",
}

enum EditionMode {
  Non = 0x00,
  Create = 0x01,
  Update = 0x02,
}

const router = useRouter();
const state = reactive({
  addCredentialDialogVisible: false,
  dialog: {
    activeCredentialKey: null,
    url: "",
    username: "",
    password: "",
    mode: EditionMode.Non,
  },
  keeperCredentialsSharedVector: new Array<KeeperCredential>(),
  lastCredentialModifications: new Set<string>(),
  validatorBox: "",
});

const credentialModalActionButtonLabel = computed(() => {
  if (state.dialog.mode === EditionMode.Create) return "Create";
  if (state.dialog.mode === EditionMode.Update) return "Save";
  return "Locked";
});

/****   UI ACTIONS  ****/
function displaySnackbar(msg: string, type: TypeMessages) {
  let sb = new Snackbar(msg, {
    position: "bottom-right",
    timeout: 5000,
  });

  let sbBackgroundColor: string = "";

  if (type === TypeMessages.Error) {
    sbBackgroundColor = "#FF5733";
  } else if (type === TypeMessages.Warning) {
    sbBackgroundColor = "#ED820E";
  } else if (type === TypeMessages.Success) {
    sbBackgroundColor = "#5F9EA0";
  } else {
    sbBackgroundColor = "#000000";
  }

  sb.setStyle({
    container: [["background-color", sbBackgroundColor]],
    message: [["color", "#ffffff"]],
  });
}

function openCredentialDialog(uniqueHashtag: string | null) {
  state.validatorBox = "";
  if (!uniqueHashtag) {
    state.dialog.mode = EditionMode.Create;
  }
  if (typeof uniqueHashtag === "string") {
    let credential: Credential | undefined =
      state.keeperCredentialsSharedVector.find(
        (item: KeeperCredential) => item.hash == uniqueHashtag
      )?.credential || undefined;

    if (!credential) {
      displaySnackbar("Credential not found", TypeMessages.Warning);
      return;
    }

    state.dialog.activeCredentialKey = uniqueHashtag;
    state.dialog.url = credential.url;
    state.dialog.username = credential.username;
    state.dialog.password = credential.password;
    state.dialog.mode = EditionMode.Update;
  }

  state.addCredentialDialogVisible = true;
}

function closeCredentialDialog() {
  state.dialog.activeCredentialKey = "";
  state.dialog.url = "";
  state.dialog.username = "";
  state.dialog.password = "";
  state.dialog.mode = EditionMode.Non;
  state.addCredentialDialogVisible = false;
}

/****   IPC ACTIONS  ****/
async function fetchSortedKeeperCredentials() {
  try {
    let heap: Array<KeeperCredential> = await invoke(
      "fetch_sorted_privacy_vec"
    );
    state.keeperCredentialsSharedVector = [...heap];
    heap = [];
  } catch (e: any) {
    displaySnackbar(e.error || "Error reading credentials", TypeMessages.Error);
  }
}

async function saveKeeperCredential() {
  let url = state.dialog.url;
  let username = state.dialog.username;
  let password = state.dialog.password;

  try {
    if (state.dialog.mode == EditionMode.Create) {
      state.dialog.activeCredentialKey = await invoke("add_privacy", {
        url,
        username,
        password,
      });
      state.keeperCredentialsSharedVector.push({
        hash: state.dialog.activeCredentialKey,
        credential: {
          url,
          username,
          password,
        },
      });
    } else if (state.dialog.mode == EditionMode.Update) {
      await invoke("update_privacy", {
        uniqueHashtag: state.dialog.activeCredentialKey,
        url,
        username,
        password,
      });

      state.keeperCredentialsSharedVector.forEach(
        (item: Credential, index: number) => {
          if (item.hash == state.dialog.activeCredentialKey) {
            state.keeperCredentialsSharedVector[index].credential = {
              url,
              username,
              password,
            };
          }
        }
      );
    } else {
      throw new Error("EditionMode Not Found");
    }
    state.lastCredentialModifications.add(state.dialog.activeCredentialKey);
    displaySnackbar("Data saved to disk", TypeMessages.Success);
    closeCredentialDialog();
  } catch (e: any) {
    state.validatorBox = e;
  }
}

async function removeKeeperCredential(uniqueHashtag: string) {
  try {
    await invoke("remove_privacy", {
      uniqueHashtag,
    });
    state.keeperCredentialsSharedVector = [
      ...state.keeperCredentialsSharedVector.filter((item: Credential) => {
        return item.hash !== uniqueHashtag;
      }),
    ];
    state.lastCredentialModifications.delete(uniqueHashtag);
    displaySnackbar("Data saved to disk", TypeMessages.Success);
  } catch (e: any) {
    displaySnackbar(
      e.error || "Error removing credentials",
      TypeMessages.Error
    );
  }
}

async function logout() {
  state.lastCredentialModifications.clear();
  state.keeperCredentialsSharedVector = [];
  await invoke("logout");
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

    <div class="credential--box-list">
      <template
        v-for="{ hash, credential } in state.keeperCredentialsSharedVector"
        :key="hash"
      >
        <CredentialBox
          :url="credential.url"
          :username="credential.username"
          :password="credential.password"
          :class="[
            state.lastCredentialModifications.has(hash)
              ? 'active-credential__box'
              : 'passive-credential__box',
          ]"
          @on-edit="
            () => {
              openCredentialDialog(hash);
            }
          "
          @on-remove="
            () => {
              removeKeeperCredential(hash);
            }
          "
        />
      </template>
    </div>

    <AddCredentialModal v-show="state.addCredentialDialogVisible">
      <template v-slot:body>
        <form
          class="add-credential__form_box"
          autocomplete="off"
          @submit.prevent="saveKeeperCredential"
        >
          <input
            v-model="state.dialog.url"
            placeholder="Enter a url..."
            autocomplete="off"
          />
          <input
            v-model="state.dialog.username"
            placeholder="Enter an username..."
            autocomplete="off"
          />
          <input
            v-model="state.dialog.password"
            placeholder="Enter a password..."
            type="password"
          />
          <div class="row">
            <button class="bluesky-effect" type="submit">
              {{ credentialModalActionButtonLabel }}
            </button>
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

.passive-credential__box {
  border: 1px solid transparent;
}
</style>