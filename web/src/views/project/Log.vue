<template>
  <a-drawer
    :title="`${data.projectName} 项目日志`"
    :maskClosable="false"
    :width="550"
    :visible="data.visible"
    @close="onClose"
  >
    <a-table
      row-key="id"
      :rowSelection="rowSelection"
      :columns="columns"
      :data-source="dataSource"
      :pagination="pagination"
      :loading="loading"
    >
      <template #bodyCell="{ column, record }">
        <template v-if="column.dataIndex === 'id'">
          #{{ record.num }}
          <a-tag color="success" v-if="record.status == 2">完成</a-tag>
          <a-tag color="processing" v-else-if="record.status == 1">构建中</a-tag>
          <a-tag color="blue" v-else-if="record.status == 0">待构建</a-tag>
          <a-tag color="red" v-else-if="record.status == -1">失败</a-tag>
          <a-tag color="warning" v-else-if="record.status == -2">停止</a-tag>

          {{ dayjs(record.createtime).format('YYYY-MM-DD HH:mm:ss') }}
        </template>
        <template v-if="column.dataIndex === 'action'">
          <a-space>
            <a-button type="default" size="small" target="_blank" :href="logUrl(record)">
              查看
            </a-button>
            <a-popconfirm
              title="确认删除?"
              ok-text="是"
              cancel-text="否"
              @confirm="delHandle(record.id)"
            >
              <a-button type="primary" size="small" danger :loading="delLoading">删除</a-button>
            </a-popconfirm>
            <a-button
              type="primary"
              size="small"
              v-if="record.status == 1"
              :loading="cancelLoading"
              @click="cancelHandle(record.project_id)"
            >
              取消
            </a-button>
            <!-- <a-button type="primary" size="small">回滚</a-button> -->
          </a-space>
        </template>
      </template>
    </a-table>
    <template #extra>
      <a-space>
        <a-button type="primary" @click="refresh">刷新</a-button>
        <a-popconfirm
          v-if="delsBtnShow"
          title="确认删除?"
          ok-text="是"
          cancel-text="否"
          @confirm="delsHandle"
        >
          <a-button type="primary" danger :loading="delsLoading">删除</a-button>
        </a-popconfirm>
      </a-space>
    </template>
  </a-drawer>
</template>
<script setup lang="ts">
import * as projectTaskApi from '@/api/project_task';
import type { ProjectLogSearch, ProjectLog } from '@/api/project_task';
import { pageRequest, useRequest } from '@/common/EasyRequest';
import { message } from 'ant-design-vue';
import type { TableProps } from 'ant-design-vue/es/table/Table';
import type { Key } from 'ant-design-vue/lib/table/interface';
import type { ComputedRef } from 'vue';
import dayjs from 'dayjs';

import { GetWebSocketEvent } from '@/common/VueSocket';
interface NotifyMessage {
  Message: string;
  Code: number;
}
const ws = GetWebSocketEvent();

ws.on('devops_notify', args => {
  const notifyMsg: NotifyMessage = JSON.parse(args);
  message.destroy();
  if (notifyMsg.Code == 2) {
    message.success('构建完成');
  }
  if (notifyMsg.Code == 1) {
    message.success('构建执行中');
  }
  if (notifyMsg.Code == -1) {
    message.error(`构建失败:${notifyMsg.Message}`);
  }
  if (notifyMsg.Code == -2) {
    message.warn(`构建取消:${notifyMsg.Message}`);
  }
  refresh();
});

const propsData = defineProps<{
  data: {
    projectId?: number;
    projectName?: string;
    visible: boolean;
  };
}>();
const data = propsData.data;
const emit = defineEmits<{
  (event: 'close'): void;
}>();
const onClose = () => {
  emit('close');
};

watch(
  () => data.visible,
  (a, b) => {
    if (a) {
      reload();
    }
  },
);
const columns = [
  {
    title: '日志概述',
    dataIndex: 'id',
    width: '100',
  },
  {
    title: '操作',
    dataIndex: 'action',
    width: '200',
  },
];

// 多选
const delsBtnShow = ref(false);
const selectedRowKeys = ref<number[]>();
const rowSelection: ComputedRef<TableProps['rowSelection']> = computed(() => {
  return {
    onChange: (ids: Key[], rows: any[]) => {
      selectedRowKeys.value = ids as number[];
      delsBtnShow.value = ids.length > 0;
    },
  };
});

// 分页列表
const searchData = ref<ProjectLogSearch>({});
const getpage = (dto: ProjectLogSearch) => {
  dto.project_id = data.projectId;
  return projectTaskApi.getLogPage(dto);
};
const { reload, refresh, loading, dataSource, pagination } = pageRequest(getpage);

// 删除单条
const { run: delHandle, loading: delLoading } = useRequest(projectTaskApi.removeById, {
  manual: true,
  onSuccess: (data, params) => {
    if (data.code != 0) {
      message.error(data.msg);
      return;
    }
    refresh();
    message.success(data.msg);
  },
});

// 删除多条
const remove = () => projectTaskApi.remove(selectedRowKeys.value);
const { run: delsHandle, loading: delsLoading } = useRequest(remove, {
  manual: true,
  onSuccess: (data, params) => {
    if (data.code != 0) {
      message.error(data.msg);
      return;
    }
    refresh();
    message.success(data.msg);
  },
});

// 取消
const { run: cancelHandle, loading: cancelLoading } = useRequest(projectTaskApi.cancel, {
  manual: true,
  onSuccess: (data, params) => {
    if (data.code != 0) {
      message.error(data.msg);
      return;
    }
    refresh();
    message.success(data.msg);
  },
});
const logUrl = (projectLog: ProjectLog) => {
  return `//${document.location.host}${import.meta.env.VITE_PROXY_HTTP_API}/log/${
    projectLog.project_id
  }/${projectLog.num}.log`;
};
</script>
