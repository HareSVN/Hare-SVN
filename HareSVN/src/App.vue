<script setup lang="ts">
import SideNav from "./components/SideNav.vue"
import MainScreen from "./components/MainScreen.vue"
import { ref } from 'vue';


const repo = ref("")
const files = ref([{status:"", fileName:""}])
</script>

<template>
  <div class="flex grid-cols-2">
    <SideNav class="" v-model:repo="repo" v-model:files="files"></SideNav>
    <MainScreen class="pl-10 pt-3" :repo="repo" :files = "files"></MainScreen>
  </div>
</template>

<style scoped>

</style>
