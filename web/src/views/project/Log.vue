<template>
  <a-modal
    v-model:open="open"
    :title="title"
    :footer="null"
    width="100%"
    :maskClosable="false"
    destroyOnClose
  >
    <div class="pt-[10px]">
      <pre class="bg-stone-100 rounded-xl p-[20px]">{{ contents }}</pre>
    </div>
  </a-modal>
</template>

<script setup lang="ts">
import { GetWebSocketEvent } from '@/api';
import { getFileName } from '.';

const ws = GetWebSocketEvent();

const open = defineModel<boolean>('open');
const props = defineProps<{
  logPath: string;
}>();

const title = computed(() => getFileName(props.logPath));

const contents = ref('');

ws.on('logData', args => {
  contents.value = args;
});

const getLog = () => ws.emit('getLog', props.logPath);

let timer: NodeJS.Timeout;

watch(
  () => open.value,
  val => {
    if (val) {
      getLog();
      timer = setInterval(() => getLog(), 1000 * 3);
      return;
    }
    clearTimeout(timer);
  },
);
</script>
