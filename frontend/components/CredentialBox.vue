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
          style="margin-right: 12px"
          width="24"
          height="24"
          @click="emit('onEdit')"
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
      password: {{ props.password }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { Credential } from "../types";
import { defineProps, defineEmits } from "vue";

const emit = defineEmits(['onEdit', 'onRemove'])
const props = defineProps<Credential>();

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