<script setup lang="ts">
import { ref, reactive } from "vue";
import { useRouter } from "vue-router";
import CredentialBox from "../components/CredentialBox.vue";
import AddCredentialModal from "../components/AddCredentialModal.vue";

const router = useRouter();

const state = reactive({
  addCredentialDialogVisible: false,
  url: "",
  name: "",
  password: "",
});

function clearCredentialDialog() {
  state.url = "";
  state.name = "";
  state.password = "";
}

function openAddCredentialDialog() {
  state.addCredentialDialogVisible = true;
}
function closeAddCredentialDialog() {
  clearCredentialDialog();  
  state.addCredentialDialogVisible = false;
}

function addCredentials() {}

function logout() {
  router.push("/");
}
</script>

<template>
  <div class="container" style="padding: 1% 2%">
    <div id="credential--toolbar">
      <button id="add_credential--button" @click="openAddCredentialDialog()">
        Add Credentials
      </button>
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

    <AddCredentialModal v-show="state.addCredentialDialogVisible">
      <template v-slot:body>
        <form class="add-credential__form_box" @submit.prevent="addCredentials">
          <input v-model="state.url" placeholder="Enter a url..." />
          <input v-model="state.name" placeholder="Enter an username..." />
          <input
            v-model="state.password"
            placeholder="Enter a password..."
            type="password"
          />
          <div class="row">
            <button class="bluesky-effect" type="submit">Create</button>
            <button
              class="vermillion-effect"
              @click="closeAddCredentialDialog()"
            >
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