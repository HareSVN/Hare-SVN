<script setup lang="ts">
import SideNav from "./components/SideNav.vue"
import MainScreen from "./components/MainScreen.vue"
import SubmitBar from "./components/SubmitBar.vue";
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri'
import { sanitize_files } from "./scripts/sanitze_files";
const repo = ref("")
const files = ref([{status:"", fileName:""}])
const selected= ref([""])
async function updateFiles(){
  console.log("UPDATING FILES 2")
  files.value = []
   let temp:Array<string>= await invoke("status", { name:repo.value });
    console.log(temp)
    temp.map((file:string)=>{
      let temp= file.split(" ").filter((character)=> character != "")
      if (temp[0].charAt(temp[0].length-1) == "?" && temp[temp.length-1] != "."){
        files.value.push({status: "Untracked", fileName: temp[temp.length-1]})
      }
      else if (temp[0].charAt(temp[0].length-1) == "A" && temp[temp.length-1] != "."){
        files.value.push({status: "Added", fileName: temp[temp.length-1]})
      }
      else if (temp[0].charAt(temp[0].length-1) == "M" && temp[temp.length-1] != "."){
        files.value.push({status: "Modified", fileName: temp[temp.length-1]})
      }
      else if (temp[0].charAt(temp[0].length-1) == "!" && temp[temp.length-1] != "."){
        files.value.push({status: "Missing", fileName: temp[temp.length-1]})
      }
      else if (temp[0].charAt(temp[0].length-1) == "D" && temp[temp.length-1] != "."){
        files.value.push({status: "Deleting", fileName: temp[temp.length-1]})
      }
      else if(temp[temp.length-1] != "." && temp[temp.length-1] != ""){
        files.value.push({status: "Up To Date", fileName: temp[temp.length-1]})
      }
      return file.split(" ").filter((character)=> character != "")
    })
    return files
}


</script>

<template>
    <div class="flex grid-cols-2">
      <SideNav class="" v-model:repo="repo" v-model:files="files"></SideNav>
      <div class="flex flex-col w-full min-h-screen justify-between">
        <MainScreen class=" pt-3" v-model:selected="selected" :repo="repo" :files = "files" @update-files="updateFiles" ></MainScreen>
      </div>
    </div>
    {{ selected }}
</template>

<style scoped>

</style>
