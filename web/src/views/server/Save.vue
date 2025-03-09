<script setup lang="ts">
import { saveServer, ServerItem } from '@/api';
import { message } from 'ant-design-vue';
import { Rule } from 'ant-design-vue/es/form';

const ishow = defineModel<boolean>('show');
const props = defineProps<{ data: ServerItem }>();
const emit = defineEmits<{
  change: [];
}>();

const form = reactive<ServerItem>({
  ip: '',
  username: '',
  remark: '',
});

watch(
  () => props.data,
  newData => {
    Object.assign(form, newData);
  },
  { immediate: true },
);

const rules: Record<string, Rule[]> = {
  ip: [{ required: true, message: '不能为空' }],
  username: [{ required: true, message: '不能为空' }],
};

const handleSave = async (values: Record<string, any>) => {
  const { code, message: msg } = await saveServer(values as ServerItem);
  if (code != 1) {
    message.warning(`保存失败：${msg}`);
    return;
  }
  message.success('保存成功');
  emit('change');
  ishow.value = false;
};
</script>

<template>
  <a-modal v-model:open="ishow" title="编辑服务器信息" :footer="null" :maskClosable="false">
    <a-form :model="form" :rules="rules" layout="vertical" @finish="handleSave">
      <a-form-item label="ip" name="ip">
        <a-input v-model:value="form.ip" />
      </a-form-item>
      <a-form-item label="账号" name="username">
        <a-input v-model:value="form.username" />
      </a-form-item>
      <a-form-item label="备注" name="remark">
        <a-input v-model:value="form.remark" />
      </a-form-item>

      <a-form-item>
        <a-space class="float-end">
          <a-button html-type="submit" type="primary">保存</a-button>
        </a-space>
      </a-form-item>
    </a-form>
  </a-modal>
</template>
