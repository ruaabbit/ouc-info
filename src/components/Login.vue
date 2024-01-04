<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const loginSuccess = ref(false);


async function login() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  loginSuccess.value = await invoke("login", {});
}
</script>

<template>
  <form class="row" @submit.prevent="login">
    <button type="submit">登录</button>
  </form>

  <p>{{ loginSuccess }}</p>
</template>
