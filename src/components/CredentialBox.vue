<template>
  <div class="credential--box">
    <div class="credential--box__item">
      <img src="../assets/icons8-account-32.png" />
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
import { defineProps } from "vue";

const props = defineProps<Credential>();
</script>

<style scoped>
.credential--box {
  width: 320px;
  height: 160px;
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
  cursor: pointer;
  will-change: filter;
  transition: 0.5s;
}
.credential--box:hover {
  filter: drop-shadow(0 0 1em #24c8db);
}
.credential--box__item {
  display: flex;
  justify-content: start;
  margin-bottom: 8px;
}
</style>