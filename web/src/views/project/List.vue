<template>
  <AButton
    type="dashed"
    block
    @click="handleAdd"
    style="width: 150px; float: right; margin-bottom: 10px"
  >
    添加项目
  </AButton>
  <ATable row-key="name" :columns="columns" :data-source="dataSource" :pagination="false">
    <template #bodyCell="{ column, record }">
      <template v-if="column.dataIndex === 'action'">
        <ASpace>
          <AButton type="primary" size="small" @click="handleEdit(record)">编辑</AButton>
          <APopconfirm title="确认删除?" ok-text="是" cancel-text="否" @confirm="handleDel(record)">
            <AButton type="primary" size="small" danger>删除</AButton>
          </APopconfirm>
          <AButton type="primary" size="small" @click="handleRun(record)">构建</AButton>
          <AButton type="primary" size="small">日志</AButton>
        </ASpace>
      </template>
    </template>
  </ATable>
</template>

<script setup lang="ts">
import { delProject, getProjects, run, type ProjectIndex } from '@/api';
import { message } from 'ant-design-vue';

const router = useRouter();

const columns = [
  {
    title: '项目文件夹',
    dataIndex: 'name',
    width: '100',
  },
  {
    title: '项目名称',
    dataIndex: 'remark',
    width: '300',
  },
  {
    title: '状态',
    dataIndex: 'status',
    width: '150',
  },
  {
    title: '构建时间',
    dataIndex: 'buildTime',
    width: '150',
  },
  {
    title: '添加时间',
    dataIndex: 'created',
    width: '200',
  },
  {
    title: '操作',
    dataIndex: 'action',
    width: '200',
  },
];

const dataSource = ref<ProjectIndex[]>([]);
const reload = async () => {
  const { data } = await getProjects();
  dataSource.value = data;
};

const handleRun = async (project: Record<string, any>) => {
  const { code, message: msg } = await run(project.name);
  if (code == 1) {
    message.success('执行成功');
    return;
  }
  message.warning(msg);
};
const handleDel = async (project: Record<string, any>) => {
  const { code, message: msg } = await delProject([project.name]);
  if (code == 1) {
    await reload();
    return;
  }
  message.warning(msg);
};
const handleAdd = () => {
  router.push({
    name: 'projectsave',
  });
};
const handleEdit = (project: Record<string, any>) => {
  router.push({
    name: 'projectsave',
    query: {
      name: project.name,
    },
  });
};

onMounted(async () => {
  await reload();
});
</script>
