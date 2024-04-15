<template>
  <div class="credential--box">
    <div class="credential--box__item">
      <div>
        <img src="../assets/icons8-account-32.png" width="24" height="24" />
      </div>
      <div>
        <img
          class="privacy-icon"
          src="../assets/privacy.png"
          style="margin-right: 16px"
          width="24"
          height="24"
          @click="emit('onEdit')"
        />
        <img
          v-if="state.passwordEyeActive"
          class="privacy-icon"
          src="../assets/hide_password.png"
          style="margin-right: 16px"
          width="24"
          height="24"
          @click="togglePassword()"
        />
        <img
          v-else
          class="privacy-icon"
          src="../assets/show_password.png"
          style="margin-right: 16px"
          width="24"
          height="24"
          @click="togglePassword()"
        />
        <img
          class="remove-icon"
          src="../assets/delete.png"
          width="24"
          height="24"
          @click="emit('onRemove')"
        />
      </div>
    </div>
    <div class="credential--box__item">{{ props.url }}</div>
    <div class="credential--box__item" style="margin-top: 8px">
      username: {{ props.username }}
    </div>
    <div class="credential--box__item" style="margin-top: 8px">
      password: {{ pwMask }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { Credential } from "../types";
import { reactive, defineProps, defineEmits, computed } from "vue";

const emit = defineEmits(["onEdit", "onRemove"]);
const props = defineProps<Credential>();

const state = reactive({
  passwordEyeActive: false,
});
const pwMask = computed(() => {
  return state.passwordEyeActive
    ? props.password
    : new Array(props.password?.length).fill("⚛").join("");
});

function togglePassword() {
  state.passwordEyeActive = !state.passwordEyeActive;
}
</script>

<style scoped>
.credential--box {
  width: 360px;
  height: 180px;
  margin: 24px 24px 0 0;
  border-radius: 8px;
  padding: 20px 15px 0 15px;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: whitesmoke;
  background-color: #0f0f0f;
  transition: border-color 0.25s;
  opacity: 0.92;
  outline: none;
  /* cursor: pointer; */
  will-change: filter;
  transition: 0.5s;
}
.credential--box:hover {
  filter: drop-shadow(0 0 1em #24c8db);
}
.credential--box__item {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
}

.privacy-icon {
  cursor: pointer;
  will-change: filter;
  transition: 0.5s;
}
.privacy-icon:hover {
  filter: drop-shadow(0 0 2.5em #ffc130);
}

.remove-icon {
  cursor: pointer;
  will-change: filter;
  transition: 0.5s;
}
.remove-icon:hover {
  filter: drop-shadow(0 2em 3em rgb(158, 86, 86));
}
</style>