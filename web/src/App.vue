<template>
  <a-config-provider
    :locale="zhCN"
    :theme="{
      algorithm: theme.defaultAlgorithm,
    }"
  >
    <router-view />
  </a-config-provider>
</template>

<script setup lang="ts">
import { theme } from 'ant-design-vue';
import zhCN from 'ant-design-vue/es/locale/zh_CN';
import dayjs from 'dayjs';
import utc from 'dayjs/plugin/utc';
import timezone from 'dayjs/plugin/timezone';
import 'dayjs/locale/zh-cn';

dayjs.extend(utc);
dayjs.extend(timezone);
dayjs.locale('zh-cn');
</script>
