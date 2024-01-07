<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const COOKIES = ref([]);


async function login() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  invoke("login_id_ouc_edu_cn");
}
</script>

<template>
  <form class="row" @submit.prevent="login">
    <button type="submit">登录</button>
  </form>
  <!-- forEach 输出 COOKIES -->
  <div v-for="cookie in COOKIES" :key="cookie.name">
    {{ cookie.name }}: {{ cookie.value }}
  </div>
</template>
