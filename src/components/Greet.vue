<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const machines = ref([]);
const ip = ref("");
const scanning = ref(false); 
const buttonText = ref("扫描");

async function scan() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  // set button to loading and disabled
  scanning.value = true;
  buttonText.value = "扫描中...";
  machines.value = await invoke("scan_machines", { ip: ip.value });
  scanning.value = false;
  buttonText.value = "扫描";
}
</script>

<template>
  <form @submit.prevent="scan">
    <input id="greet-input" v-model="ip" placeholder="输入机器IP..." />
    <el-button type="primary" round :disabled="scanning">{{ buttonText }}</el-button>
  </form>

  <p>{{ machines }}</p>
</template>
