<script setup lang="ts">
import SideNav from "./components/SideNav.vue"
import MainScreen from "./components/MainScreen.vue"
import { ref } from 'vue';


const repo = ref("")
</script>

<template>
  <div >
    <SideNav v-model:repo="repo"></SideNav>
    <MainScreen :repo="repo"></MainScreen>
  </div>
</template>

<style scoped>

</style>
