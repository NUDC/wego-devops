<script setup lang="ts">
import {
  getProjectInfo,
  ProjectInfo,
  setProjectInfo,
  getLogs,
  build,
  deploy,
  run,
  getServers,
  removeLog,
} from '@/api';
import { message } from 'ant-design-vue';
import { Rule } from 'ant-design-vue/es/form';
import Log from './Log.vue';
import { getFileName } from '.';

const router = useRouter();
const route = useRoute();
const id = computed(() => {
  return {
    name: route.query?.name?.toString() ?? '',
    group: route.query?.group?.toString() ?? '',
  };
});
const isEdit = computed(() => id.value.name !== '');
const logOpen = ref(false);
const logPath = ref('');

const form = reactive<ProjectInfo>({
  name: '',
  remark: '',
  buildScript: '',
  deploy: [],
});
const rules: Record<string, Rule[]> = {
  group: [{ required: true, message: '请输入项目分组' }],
  name: [{ required: true, message: '请输入项目文件夹' }],
  remark: [{ required: true, message: '请输入项目名称' }],
  buildScript: [{ required: true, message: '请输入构建脚本' }],
};
const logsData = ref<string[]>([]);
const reloadLogs = async () => {
  if (isEdit.value === false) {
    logsData.value = [];
    return;
  }

  const { data } = await getLogs(id.value);
  logsData.value = data;
};

const handleShowLog = (path: string) => {
  logPath.value = path;
  logOpen.value = true;
};
const handleRemoveLog = async (path: string) => {
  const { code } = await removeLog(path);
  if (code != 1) {
    message.warning(`删除日志失败`);
    return;
  }
  await reloadLogs();
  message.success('删除日志成功');
};

const handleSave = async (values: ProjectInfo) => {
  const { code, message: msg } = await setProjectInfo(values as ProjectInfo);
  if (code != 1) {
    message.warning(`保存失败：${msg}`);
    return;
  }
  router.push({
    name: 'projectsave',
    query: {
      name: values.name,
      group: values.group,
    },
  });
  message.success('保存成功');
};
const handleBulder = async () => {
  const { code, message: msg } = await build(id.value);
  if (code == 1) {
    message.success('执行成功');
    return;
  }
  message.warning(msg);
};
const hanldleDeploy = async (c: string) => {
  const { code, message: msg } = await deploy({
    codes: [c],
    ...id.value,
  });
  if (code == 1) {
    message.success('执行成功');
    return;
  }
  message.warning(msg);
};
const handleRun = async () => {
  const { code, message: msg } = await run({
    codes: [],
    ...id.value,
  });
  if (code == 1) {
    message.success('执行成功');
    return;
  }
  message.warning(msg);
};

const addDeploy = () => {
  form.deploy.push({
    code: '',
    ip: '',
    deployScript: '',
  });
};

const removeDeploy = (index: number) => {
  form.deploy.splice(index, 1);
};

const serverSelectOptions = ref<{ label: string; value: string }[]>([]);

let timer: NodeJS.Timeout;
onBeforeMount(async () => {
  const { data: serverList } = await getServers();
  serverSelectOptions.value = serverList.map(o => {
    return { label: `${o.remark}(${o.ip})`, value: o.ip };
  });

  if (isEdit.value === false) {
    return;
  }
  const { data } = await getProjectInfo(id.value);
  Object.assign(form, data);
  await reloadLogs();
  timer = setInterval(async () => {
    await reloadLogs();
  }, 1000 * 3);
});

onUnmounted(() => {
  clearInterval(timer);
});
</script>

<template>
  <div class="flex justify-between">
    <div class="flex flex-col gap-3 mr-[50px] w-[250px]">
      <AButton type="primary" @click="handleBulder" v-show="isEdit">构建</AButton>
      <APopconfirm
        title="确认执行构建+部署所有脚本?"
        description="构建成功后将部署所有已配置的部署脚本"
        ok-text="是"
        cancel-text="否"
        @confirm="handleRun"
      >
        <AButton type="primary" v-show="isEdit">构建+部署</AButton>
      </APopconfirm>
      <div>
        <a-divider orientation="center"><span>日志</span></a-divider>
        <div>
          <template v-for="item in logsData">
            <div class="flex gap-1.5 justify-between items-center mb-1">
              <a-button type="text" @click="handleShowLog(item)">
                {{ getFileName(item) }}
              </a-button>
              <a-button type="link" size="small" danger @click="handleRemoveLog(item)">
                删除
              </a-button>
            </div>
          </template>
        </div>
      </div>
    </div>
    <div class="w-full">
      <a-form :model="form" :rules="rules" layout="vertical" @finish="handleSave">
        <a-form-item label="项目分组" name="group">
          <a-input v-model:value="form.group" :disabled="isEdit" />
        </a-form-item>
        <a-form-item label="项目文件夹" name="name">
          <a-input v-model:value="form.name" :disabled="isEdit" />
        </a-form-item>

        <a-form-item label="项目名称" name="remark">
          <a-input v-model:value="form.remark" />
        </a-form-item>
        <a-divider orientation="left">构建</a-divider>
        <a-form-item label="构建脚本" name="buildScript">
          <CodeEditor v-model="form.buildScript" />
        </a-form-item>

        <a-divider orientation="left">部署</a-divider>
        <a-form-item name="deploy">
          <div class="mb-[30px]" v-for="(item, index) in form.deploy" :key="index">
            <a-card :title="`部署脚本${index + 1}`">
              <template #extra>
                <a-space>
                  <a-button
                    v-show="isEdit"
                    type="primary"
                    size="small"
                    @click="hanldleDeploy(item.code)"
                  >
                    部署
                  </a-button>
                  <APopconfirm
                    title="确认删除?"
                    ok-text="是"
                    cancel-text="否"
                    @confirm="removeDeploy(index)"
                  >
                    <a-button danger type="dashed" size="small">删除</a-button>
                  </APopconfirm>
                </a-space>
              </template>

              <a-form layout="vertical" :model="item">
                <a-form-item label="唯一编号" :name="['deploy', index, 'code']">
                  <a-input v-model:value="item.code" style="width: 100%" />
                </a-form-item>
                <a-form-item label="服务器" :name="['deploy', index, 'ip']">
                  <a-select
                    v-model:value="item.ip"
                    style="width: 100%"
                    :options="serverSelectOptions"
                  ></a-select>
                </a-form-item>
                <a-form-item label="部署脚本" :name="['deploy', index, 'deployScript']">
                  <CodeEditor v-model="item.deployScript" />
                </a-form-item>
              </a-form>
            </a-card>
          </div>
          <a-button class="float-right" type="dashed" size="small" @click="addDeploy">
            添加部署配置
          </a-button>
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
