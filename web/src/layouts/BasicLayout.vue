<template>
  <a-layout>
    <a-layout-header>
      <div
        class="float-left w-[150px] h-[45px] m-[10px_10px_10px_0] bg-slate-400 rounded-[2px] text-blue-100 text-[20px] font-black flex justify-center items-center"
      >
        wego devops
      </div>
      <Menu />
    </a-layout-header>
    <a-layout-content class="px-[50px]">
      <div class="my-[16px]">
        <Breadcrumb />
      </div>
      <div class="bg-[#fff] p-[24px] min-h-screen">
        <router-view />
      </div>
    </a-layout-content>
  </a-layout>
</template>

<script lang="ts" setup>
import Menu from './components/Menu.vue';
import Breadcrumb from './components/Breadcrumb.vue';
</script>
