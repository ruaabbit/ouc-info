<script setup>
import { ref, defineModel } from "vue";
import { useRoute } from 'vue-router'
import { invoke } from "@tauri-apps/api/tauri";

const route = useRoute();


var pdfUrl = ref("");
var cookies = route.query.cookies ? JSON.parse(route.query.cookies) : [];
var stuId = defineModel();
const iframeUrl = ref("");

function getScorePdfUrl() {
    invoke("get_score_pdf_url", { cookies: cookies, stuId: stuId.value })
        .then((url) => {
            pdfUrl.value = url;
            invoke("get_pdf_blob", { cookies: cookies, pdfUrl: url })
                .then((res) => {
                    const blob = new Blob([new Uint8Array(res)], { type: "application/pdf" });
                    iframeUrl.value = window.URL.createObjectURL(blob);
                })
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

        <form class="row" @submit.prevent="getScorePdfUrl">
            <label for="stuId">学号 </label>
            <input type="text" id="stuId" v-model="stuId" />
            <button type="submit">查看成绩单</button>
        </form>
        <div v-if="pdfUrl">
            <iframe :src="iframeUrl" style="position:relative;" width="100%" height="600px" />

        </div>
    </div>
</template>


