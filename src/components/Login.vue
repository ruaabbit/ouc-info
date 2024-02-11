<script setup>
import { ref } from "vue";
import { useRouter } from 'vue-router'
import { invoke } from "@tauri-apps/api/tauri";

const cookies = ref([]);
const router = useRouter();

async function login() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  invoke("login_id_ouc_edu_cn")
    .then((cookies) => {
      cookies.value = cookies;
      // 获取到cookies后，跳转到main页面  
      router.push({ path: '/main', query: { cookies: JSON.stringify(cookies) } });
    })
    .catch((error) => {
      alert(error);
    })
}
</script>

<template>
  <div class="container">
    <h1>Welcome to Tauri!</h1>

    <div class="row">
      <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      <img src="../assets/vue.svg" class="logo vue" alt="Vue logo" />
    </div>

    <form class="row" @submit.prevent="login">
      <button type="submit">登录</button>
    </form>
    <!-- forEach 输出 COOKIES -->
    <div v-for="cookie in cookies" :key="cookie.name">
      {{ cookie.name }}: {{ cookie.value }}
    </div>

  </div>
</template>
