<template>
  <div id="app">
    <h2>Verificador de Estado de Repositorios Git</h2>
    <Settings v-model="repos" />
    <div>
      <button @click="checkStatus">Verificar Estado</button>
    </div>
    <div class="container" v-if="message" :class="{ error: hasError }">
      {{ message }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import Settings from "./components/Settings.vue";

const repos: Ref<string[]> = ref([]);
const message = ref("");
const hasError = ref(false);

const checkStatus = async () => {
  try {
    const result = await invoke("check_git_status", { repos: repos.value });
    message.value = result;
    hasError.value = false;
  } catch (error) {
    message.value = error;
    hasError.value = true;
  }
};
</script>
