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
import { getLog } from '@/api';
import { getFileName } from '.';

const open = defineModel<boolean>('open');
const props = defineProps<{
  logPath: string;
}>();
watch(
  () => props.logPath,
  async val => {
    await getLogContent(val);
  },
);

const title = computed(() => getFileName(props.logPath));

const contents = ref('');

const getLogContent = async (logPath: string) => {
  const { data } = await getLog(logPath);
  contents.value = data;
};

// import { GetWebSocketEvent } from '@/api';
// import { message } from 'ant-design-vue';
// interface NotifyMessage {
//   Message: string;
//   Code: number;
// }
// const ws = GetWebSocketEvent();

// ws.on('devops_notify', args => {
//   const notifyMsg: NotifyMessage = JSON.parse(args);
//   message.destroy();
//   if (notifyMsg.Code == 2) {
//     message.success('构建完成');
//   }
//   if (notifyMsg.Code == 1) {
//     message.success('构建执行中');
//   }
//   if (notifyMsg.Code == -1) {
//     message.error(`构建失败:${notifyMsg.Message}`);
//   }
//   if (notifyMsg.Code == -2) {
//     message.warn(`构建取消:${notifyMsg.Message}`);
//   }
// });
</script>
