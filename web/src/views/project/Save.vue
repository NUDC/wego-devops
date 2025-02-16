<template>
  <a-form :model="form" :rules="rules" layout="vertical" @finish="handleSave">
    <a-form-item label="项目文件夹" name="name">
      <a-input v-model:value="form.name" :disabled="isReadonly" />
    </a-form-item>
    <a-form-item label="项目名称" name="remark">
      <a-input v-model:value="form.remark" />
    </a-form-item>
    <a-form-item label="构建脚本" name="buildScript">
      <CodeEditor v-model="form.buildScript" type="shell" />
    </a-form-item>
    <a-form-item label="部署脚本" name="deployScript">
      <CodeEditor v-model="form.deployScript" type="shell" />
    </a-form-item>
    <a-form-item>
      <a-space>
        <a-button @click="() => router.go(-1)">返回</a-button>
        <a-button html-type="submit" type="primary">保存</a-button>
      </a-space>
    </a-form-item>
  </a-form>
</template>
<script setup lang="ts">
import { getProjectInfo, ProjectInfo, setProjectInfo } from '@/api';
import { message } from 'ant-design-vue';
import { Rule } from 'ant-design-vue/es/form';

const router = useRouter();
const route = useRoute();
const name = route.query?.name?.toString();
const isReadonly = computed(() => name != undefined);

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

const handleSave = async (values: ProjectInfo) => {
  console.log(values);
  const { code, message: msg } = await setProjectInfo(values as ProjectInfo);
  if (code != 1) {
    message.warning(`保存失败：${msg}`);
    return;
  }
  message.success('保存成功');
};

onBeforeMount(async () => {
  if (name === undefined) {
    return;
  }
  const { data } = await getProjectInfo(name);
  form.name = data.name;
  form.remark = data.remark;
  form.buildScript = data.buildScript;
  form.deployScript = data.deployScript;
});
</script>
