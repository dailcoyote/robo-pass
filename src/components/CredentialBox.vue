<template>
  <div class="credential--box">
    <div class="credential--box__item">goggle.com</div>
    <div class="credential--box__item" style="margin-top: 8px">
      username: pumpitkit@yahoo.com
    </div>
    <div class="credential--box__item" style="margin-top: 8px">
      password: kdk88888888
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "CredentialBox",
});
</script>

<style scoped>
.credential--box {
  width: 320px;
  height: 160px;
  margin: 24px 24px 0 0;
  border-radius: 8px;
  padding: 18px 12px 0 12px;
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