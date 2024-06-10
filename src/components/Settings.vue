<template>
  <div>
    <h4>Configuración de Repositorios</h4>
    <ul class="list container">
      <li v-for="(repo, index) in repos" :key="index">
        {{ repo }}
        <button @click="removeRepo(index)">Eliminar</button>
      </li>
    </ul>
    <div class="fl fl-end">
      <button @click="openDialog">Seleccionar Repositorio</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, defineProps, defineEmits, onMounted } from "vue";
import { open } from "@tauri-apps/api/dialog";

const props = defineProps({
  modelValue: {
    type: Array,
    default: () => [],
  },
});
const emit = defineEmits(["update:modelValue"]);

const repos = ref([...props.modelValue]);

const findRepos = () => {
  const storedRepos = window.localStorage.getItem("repos");
  if (storedRepos) {
    repos.value = JSON.parse(storedRepos);
    emit("update:modelValue", repos.value);
  }
};

onMounted(() => {
  findRepos();
});

const openDialog = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (selected) {
      repos.value.push(selected);
      emit("update:modelValue", repos.value);
      window.localStorage.setItem("repos", JSON.stringify(repos.value));
    }
  } catch (error) {
    console.error("Error al seleccionar el directorio:", error);
  }
};

const removeRepo = (index: number) => {
  repos.value.splice(index, 1);
  emit("update:modelValue", repos.value);
  window.localStorage.setItem("repos", JSON.stringify(repos.value));
};
</script>
