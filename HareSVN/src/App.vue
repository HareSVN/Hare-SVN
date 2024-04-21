<script setup lang="ts">
import SideNav from "./components/SideNav.vue"
import MainScreen from "./components/MainScreen.vue"
import SubmitBar from "./components/SubmitBar.vue"
import { ref } from 'vue';


const repo = ref("")
</script>

<template>
  <div >
    <SideNav v-model:repo="repo"></SideNav>
    <MainScreen :repo="repo"></MainScreen>
    <SubmitBar></SubmitBar>
  </div>
</template>

<style scoped>

</style>
