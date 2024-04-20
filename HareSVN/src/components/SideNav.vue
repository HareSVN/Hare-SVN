<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri'
const temp = ref(["one", "two", "three"])
const input = ref("")
const result = ref("")
const active = ref("")
async function addRepo(e: { preventDefault: () => void; }){
  e.preventDefault()
  temp.value.push(input.value)
  result.value = await invoke("greet", { name: input.value });
  input.value = ""
}
function changeActive(item){
  active.value = item
  let isActive = "bg-red-200"
  if (active ==item)
      isActive = "bg-blue-200"
}

function setItemCSS(item){
  let isActive = "bg-red-200"
  console.log(active, item)
  if (active.value == item)
      isActive = "bg-blue-200"

  return `block w-full bg-gray-300 text-center ring-1 my-3 text-lg font-bold ${isActive}`
}
</script>

<template>
    <div class="min-h-screen w-[20%] bg-gray-100">
      <form v-on:submit="addRepo">
        <div class="flex mt-2 text-center bg-gray-200">
          <label for="email" class="block font-medium text-sm text-gray text-center">Enter Repository URL</label>
          <div class=" flex mt-1 my-2 px-2 justify-center">
            <input id="repo" name="repo" v-model = "input" type="text" required class="block w-full bg-gray-100 rounded-md py-1 pl-2 mt-2 ring-1 ring-gray-400 shadow-md {{ if active == item ? 'text-blue-100' :'text-red-400'}}">
          </div>
        </div>
      </form>
      <hr>
      <div>
        <div v-for="item in temp" :key="item" @click="changeActive(item)" :class="setItemCSS(item)">{{item}}</div>
      </div>
      {{active }}
    </div>

</template>