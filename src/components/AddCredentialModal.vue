<template>
  <div id="credential--modal_cover">
    <div
      id="credential--modal__content"
      role="dialog"
      aria-labelledby="credential--modal__content_title"
      aria-describedby="credential--modal__content_body"
    >
      <header id="credential--modal__content_title">
        <slot name="header">
          <h3>Add Credential</h3>
        </slot>
      </header>

      <section id="credential--modal__content_body">
        <slot name="body"> Body </slot>
      </section>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "AddCredentialModal",
});
</script>


<style scoped>
#credential--modal_cover {
  position: fixed;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  background-color: rgba(0, 0, 0, 0.3);
  display: flex;
  justify-content: center;
  z-index: 666;
}

#credential--modal__content {
  margin: 8%;
  box-shadow: 2px 2px 20px 1px;
  overflow-x: auto;
  display: flex;
  flex-direction: column;
  width: 600px;
  height: 600px;
  background: no-repeat url('../assets/lord.jpg');
  background-size: cover;
}

#credential--modal__content_body {
  position: relative;
}
</style>