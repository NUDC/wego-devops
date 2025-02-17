<template>
  <div class="flex justify-between">
    <div class="flex flex-col gap-3 pr-[50px] w-[200px]">
      <AButton type="primary" @click="handleRun" v-show="isEdit">构建</AButton>

      <div>
        <a-divider orientation="center"><span>日志</span></a-divider>
        <a-list item-layout="horizontal" :data-source="logsData">
          <template #renderItem="{ item }">
            <a-button type="text" @click="handleShowLog(item)">{{ getFileName(item) }}</a-button>
          </template>
        </a-list>
      </div>
    </div>
    <div class="w-full">
      <a-form :model="form" :rules="rules" layout="vertical" @finish="handleSave">
        <a-form-item label="项目文件夹" name="name">
          <a-input v-model:value="form.name" :disabled="isEdit" />
        </a-form-item>
        <a-form-item label="项目名称" name="remark">
          <a-input v-model:value="form.remark" />
        </a-form-item>
        <a-form-item label="构建脚本" name="buildScript">
          <CodeEditor v-model="form.buildScript" />
        </a-form-item>
        <a-form-item label="部署脚本" name="deployScript">
          <CodeEditor v-model="form.deployScript" />
        </a-form-item>
        <a-form-item>
          <a-space>
            <a-button @click="() => router.go(-1)">返回</a-button>
            <a-button html-type="submit" type="primary">保存</a-button>
          </a-space>
        </a-form-item>
      </a-form>
    </div>
    <Log v-model:open="logOpen" :logPath="logPath" />
  </div>
</template>
<script setup lang="ts">
import { getProjectInfo, ProjectInfo, run, setProjectInfo, getLogs } from '@/api';
import { message } from 'ant-design-vue';
import { Rule } from 'ant-design-vue/es/form';
import Log from './Log.vue';
import { getFileName } from '.';

const router = useRouter();
const route = useRoute();
const name = computed(() => route.query?.name?.toString() ?? '');
const isEdit = computed(() => name.value !== '');
const logOpen = ref(false);
const logPath = ref('');

const form = reactive<ProjectInfo>({
  name: '',
  remark: '',
  buildScript: '',
  deployScript: '',
});
const rules: Record<string, Rule[]> = {
  name: [{ required: true, message: '不能为空' }],
  remark: [{ required: true, message: '不能为空' }],
  buildScript: [{ required: true, message: '不能为空' }],
  deployScript: [{ required: true, message: '不能为空' }],
};
const logsData = ref<string[]>([]);
const reloadLogs = async () => {
  if (isEdit.value === false) {
    logsData.value = [];
    return;
  }
  const { data } = await getLogs(name.value);
  logsData.value = data;
};

const handleShowLog = (path: string) => {
  logPath.value = path;
  logOpen.value = true;
};

const handleSave = async (values: ProjectInfo) => {
  console.log(values);
  const { code, message: msg } = await setProjectInfo(values as ProjectInfo);
  if (code != 1) {
    message.warning(`保存失败：${msg}`);
    return;
  }
  message.success('保存成功');
};
const handleRun = async () => {
  if (isEdit.value === false) {
    return;
  }
  const { code, message: msg } = await run(name.value);
  if (code == 1) {
    message.success('执行成功');
    return;
  }
  message.warning(msg);
  // setTimeout(async () => await reloadLogs(), 1000 * 3);
};

onBeforeMount(async () => {
  console.log(name.value);
  if (name.value === '') {
    return;
  }
  const { data } = await getProjectInfo(name.value);
  form.name = data.name;
  form.remark = data.remark;
  form.buildScript = data.buildScript;
  form.deployScript = data.deployScript;

  await reloadLogs();
});
</script>
