<script setup lang="ts">
import { build, delProject, getProjects, run, type ProjectIndex } from '@/api';
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
const groups = computed(() => [...new Set(dataSource.value.map(o => o.group))]);
const group = ref<string>();
const groupSouce = computed(() => dataSource.value.filter(o => o.group === group.value));

const reload = async () => {
  const { data } = await getProjects();
  dataSource.value = data;
};

const handleRun = async (record: Record<string, any>) => {
  const { code, message: msg } = await run({
    ...getId(record),
    codes: [],
  });
  if (code == 1) {
    message.success('执行成功');
    return;
  }
  message.warning(msg);
};
const handleBuild = async (record: Record<string, any>) => {
  const { code, message: msg } = await build({
    ...getId(record),
  });
  if (code == 1) {
    message.success('执行成功');
    return;
  }
  message.warning(msg);
};
const handleDel = async (record: Record<string, any>) => {
  const { code, message: msg } = await delProject([getId(record)]);
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
const handleEdit = (record: Record<string, any>) => {
  router.push({
    name: 'projectsave',
    query: {
      ...getId(record),
    },
  });
};

function getId(record: Record<string, any>) {
  return { name: record.name, group: record.group };
}

onMounted(async () => {
  await reload();
  group.value = groups.value.find(() => true);
});
</script>

<template>
  <div class="flex gap-3 justify-end py-2">
    <AButton type="dashed" @click="handleAdd">添加项目</AButton>
  </div>
  <a-tabs type="card" v-model:activeKey="group">
    <a-tab-pane v-for="item in groups" :key="item" :tab="item"></a-tab-pane>
  </a-tabs>
  <ATable row-key="name" :columns="columns" :data-source="groupSouce" :pagination="false">
    <template #bodyCell="{ column, record }">
      <template v-if="column.dataIndex === 'action'">
        <ASpace>
          <AButton type="primary" size="small" @click="handleEdit(record)">编辑</AButton>
          <AButton type="primary" size="small" @click="handleBuild(record)">构建</AButton>
          <APopconfirm
            title="确认执行构建+部署所有脚本?"
            description="构建成功后将部署所有已配置的部署脚本"
            ok-text="是"
            cancel-text="否"
            @confirm="handleRun(record)"
          >
            <AButton type="primary" size="small">构建+部署</AButton>
          </APopconfirm>
          <APopconfirm title="确认删除?" ok-text="是" cancel-text="否" @confirm="handleDel(record)">
            <AButton type="primary" size="small" danger>删除</AButton>
          </APopconfirm>
        </ASpace>
      </template>
    </template>
  </ATable>
</template>
