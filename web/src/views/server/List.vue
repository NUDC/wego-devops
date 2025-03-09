<script setup lang="ts">
import { delServers, getServers, ServerItem } from '@/api';

import { message } from 'ant-design-vue';
import Save from './Save.vue';

const formData = ref<{
  ishow: boolean;
  data: ServerItem;
}>({
  ishow: false,
  data: {
    ip: '',
    username: '',
    privateKey: '',
    remark: '',
  },
});

const handleAdd = () => {
  formData.value.ishow = true;
  formData.value.data = {
    ip: '',
    username: '',
    privateKey: '',
    remark: '',
  };
};
const handleEdit = (model: Record<string, any>) => {
  formData.value.ishow = true;
  formData.value.data = model as ServerItem;
};

const columns = [
  {
    title: 'ip',
    dataIndex: 'ip',
    width: '150',
  },
  {
    title: '账号',
    dataIndex: 'username',
    width: '100',
  },
  {
    title: '备注',
    dataIndex: 'remark',
    width: '200',
  },
  {
    title: '操作',
    dataIndex: 'action',
    width: '200',
  },
];

const dataSource = ref<any[]>([]);
const reload = async () => {
  const { data } = await getServers();
  dataSource.value = data;
};

const handleDel = async (model: Record<string, any>) => {
  const { code, message: msg } = await delServers([model.ip]);
  if (code == 1) {
    await reload();
    return;
  }
  message.warning(msg);
};

onMounted(async () => {
  await reload();
});
</script>

<template>
  <div class="flex gap-3 justify-end py-2">
    <AButton type="dashed" @click="handleAdd">添加服务器</AButton>
  </div>
  <ATable row-key="ip" :columns="columns" :data-source="dataSource" :pagination="false">
    <template #bodyCell="{ column, record }">
      <template v-if="column.dataIndex === 'action'">
        <ASpace>
          <AButton type="primary" size="small" @click="handleEdit(record)">编辑</AButton>
          <APopconfirm title="确认删除?" ok-text="是" cancel-text="否" @confirm="handleDel(record)">
            <AButton type="primary" size="small" danger>删除</AButton>
          </APopconfirm>
        </ASpace>
      </template>
    </template>
  </ATable>

  <Save v-model:show="formData.ishow" :data="formData.data" @change="reload" />
</template>
