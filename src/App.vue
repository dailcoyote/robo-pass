<template>
  <router-view />
  <teleport to="body">
    <vue3-snackbar bottom right :duration="10000"></vue3-snackbar>
  </teleport>
</template>