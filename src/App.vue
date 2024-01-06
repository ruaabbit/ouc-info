<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Login from "./components/Login.vue";
</script>

<template>
  <div class="container">
    <h1>Welcome to Tauri!</h1>

    <div class="row">
      <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
    </div>

    <Login />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
