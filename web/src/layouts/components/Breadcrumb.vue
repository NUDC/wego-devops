<template>
  <a-breadcrumb>
    <a-breadcrumb-item v-for="item in routes" :key="item.path" :href="item.path">
      <span>{{ item.breadcrumbName }}</span>
    </a-breadcrumb-item>
  </a-breadcrumb>
</template>

<script lang="ts" setup>
import { RouterMeta } from '@/router/typing';
import { Route } from 'ant-design-vue/es/breadcrumb/Breadcrumb';

interface BreadcrumbItem extends Route {
  icon?: string;
}

const router = useRouter();
const routes = computed(() =>
  router.currentRoute.value.matched.concat().map(item => {
    const meta = item.meta as unknown as RouterMeta;
    const route: BreadcrumbItem = {
      path: item.path,
      breadcrumbName: meta.title,
      icon: meta.icon,
    };
    return route;
  }),
);
</script>
