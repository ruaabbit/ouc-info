<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const loginSessionId = ref("");


async function login() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  loginSessionId.value = await invoke("login_id_ouc_edu_cn", {});
}
</script>

<template>
  <form class="row" @submit.prevent="login">
    <button type="submit">登录</button>
  </form>

  <p>JSESSIONID : {{ loginSessionId }}</p>
</template>
