<script setup lang="ts">
import SideNav from "./components/SideNav.vue"
import MainScreen from "./components/MainScreen.vue"
import SubmitBar from "./components/SubmitBar.vue";
import { ref } from 'vue';


const repo = ref("")
const files = ref([{status:"", fileName:""}])
const selected= ref([""])
</script>

<template>
    <div class="flex grid-cols-2">
      <SideNav class="" v-model:repo="repo" v-model:files="files"></SideNav>
      <div class="flex flex-col w-full min-h-screen justify-between">
        <MainScreen class=" pt-3" v-model:selected="selected" :repo="repo" :files = "files" ></MainScreen>
      </div>
    </div>
    {{ selected }}
</template>

<style scoped>

</style>
