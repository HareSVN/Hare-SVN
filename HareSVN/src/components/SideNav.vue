<script setup lang="ts">
import PopUpFile from "./PopUpFile.vue"
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri'
const temp = ref<Array<{url:string, name:string}>>([])
const input = ref({
  url: "",
  name: "",
})
const emit =defineEmits(['updateFiles'])
const checkout = ref("")
const result = defineModel("files", {default: [{status:"", fileName:""}]})
const active = defineModel("repo")
//const fileName = ref("")
async function addRepo(e: { preventDefault: () => void; }){
  e.preventDefault()
  input.value.name = input.value.url.substring(input.value.url.lastIndexOf("/") + 1, input.value.url.length)
  temp.value.push(input.value)
  checkout.value = await invoke("checkout", { url: input.value.url, name: input.value.name });
  input.value = {url: "", name:""}
}
const revision = ref("")
const dir = ref("test")
function updateFiles(){
  console.log("UPDATING FILES")
  emit('updateFiles')
}
async function displayRevision(){
  revision.value = await invoke("revision", {name: active.value})
}
async function createLog(){
  await invoke('history', {name:active.value})
}
async function changeActive(item:string){
  displayRevision()
  active.value = item
  result.value = []
   let temp:Array<string>= await invoke("status", { name:item });
    console.log(temp)
   temp.map((file:string)=>{
      let temp= file.split(" ").filter((character)=> character != "")
      if (temp[0].charAt(temp[0].length-1) == "?" && temp[temp.length-1] != "."){
        result.value.push({status: "Untracked", fileName: temp[temp.length-1]})
      }
      else if (temp[0].charAt(temp[0].length-1) == "A" && temp[temp.length-1] != "."){
        result.value.push({status: "Added", fileName: temp[temp.length-1]})
      }
      else if (temp[0].charAt(temp[0].length-1) == "M" && temp[temp.length-1] != "."){
        result.value.push({status: "Modified", fileName: temp[temp.length-1]})
      }
      else if (temp[0].charAt(temp[0].length-1) == "!" && temp[temp.length-1] != "."){
        result.value.push({status: "Missing", fileName: temp[temp.length-1]})
      }
      else if (temp[0].charAt(temp[0].length-1) == "D" && temp[temp.length-1] != "."){
        result.value.push({status: "Deleting", fileName: temp[temp.length-1]})
      }
      else if(temp[temp.length-1] != "." && temp[temp.length-1] != ""){
        result.value.push({status: "Up To Date", fileName: temp[temp.length-1]})
      }
      return file.split(" ").filter((character)=> character != "")
    })
}
const popup = ref(false)
function changePopup(){
  popup.value = !popup.value
}
function setItemCSS(item:string){
  let isActive = "bg-red-200"
  console.log(active, item)
  if (active.value == item)
      isActive = "bg-blue-200"

  return `flex justify-between w-full bg-gray-300 text-center ring-1 my-3 text-lg font-bold ${isActive}`
}

async function updateRepo() {
  await invoke('update', {name:active.value})
}

</script>

<template>
    <div class="min-h-screen min-w-[20%] bg-gray-100 border-r-2">
      <form v-on:submit="addRepo">
        <div class="flex text-center bg-gray-200">
          <label for="repo" class="block font-bold text-sm text-center">Enter Repository URL</label>
          <div class="flex mt-1 my-2 px-2 justify-center">
            <input id="repo" name="repo" v-model = "input.url" type="text" required class="block w-full bg-gray-100 rounded-md pl-1 m-1 shadow-md">
          </div>
        </div>
      </form>
      <hr>
      <div>
        <p class="block font-bold text-left pl-2 pt-2">Revision Number: {{ revision }}</p>
      </div>
      <div>
        <div v-for="item in temp" :key="item.name" @click="changeActive(item.name)" :class="setItemCSS(item.name)">
          <div class="pl-5">
            {{item.name}}
          </div>
          <div class="flex items-end">
            <div class="pr-2 cursor-pointer" @click="changePopup">➕</div>
            <div class="pr-2 cursor-pointer" @click="createLog">⏬</div>
            <div class="pr-2 cursor-pointer" @click="updateRepo">🔄</div>
          </div>
        </div>
      </div>
      <PopUpFile v-if="popup" v-model:popup="popup" v-model:dir="dir" :repo="active" @update-files="updateFiles"/>
    </div>
</template>