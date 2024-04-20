<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri'
const temp = ref(["one", "two", "three"])
const input = ref("")
const result = ref("")
async function addRepo(e: { preventDefault: () => void; }){
  e.preventDefault()
  temp.value.push(input.value)
  input.value = ""
  result.value = await invoke("checkout", { name: input.value });
}
</script>

<template>
    <div class="min-h-screen w-[20%] bg-gray-100">
      <form v-on:submit="addRepo">
        <div class="flex mt-2 text-center bg-gray-200">
          <label for="email" class="block font-medium text-sm text-gray text-center">Enter Repository URL</label>
          <div class=" flex mt-1 my-2 px-2 justify-center">
            <input id="repo" name="repo" v-model = "input" type="text" required class=" block bg-gray-100 rounded-md py-1 pl-2 mt-2 ring-1 ring-gray-400 shadow-md ">
          </div>
        </div>
      </form>
      <hr>
      <div>
        <li v-for="item in temp" :key="item">{{item}}</li>
      </div>
      {{  result }}
    </div>

</template>