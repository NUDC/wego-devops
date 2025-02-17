<template>
  <a-layout class="layout">
    <a-layout-header>
      <div class="logo">wego devops</div>
      <Menu />
    </a-layout-header>
    <a-layout-content style="padding: 0 50px">
      <Breadcrumb style="margin: 16px 0" />
      <div :style="{ background: '#fff', padding: '24px', minHeight: '100vh' }">
        <router-view />
      </div>
    </a-layout-content>
  </a-layout>
</template>

<script lang="ts" setup>
import Menu from './components/Menu.vue';
import Breadcrumb from './components/Breadcrumb.vue';
</script>

<style scoped>
.logo {
  float: left;
  width: 150px;
  height: 45px;
  margin: 10px 10px 10px 0;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 2px;
  color: lightblue;
  font-weight: bolder;
  font-size: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
