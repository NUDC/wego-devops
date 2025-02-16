<template>
  <a-menu
    v-model:selectedKeys="state.selectedKeys"
    :items="menuItems"
    theme="dark"
    mode="horizontal"
  />
</template>

<script lang="ts" setup>
import { RouterMeta } from '@/router/typing';
import { RouteRecordRaw } from 'vue-router';

const router = useRouter();
const routes = router.getRoutes();

const state = reactive({
  openKeys: [],
  selectedKeys: [],
});

watch(
  router.currentRoute,
  () => {
    const matched = router.currentRoute.value.matched.concat();
    state.selectedKeys = matched.filter(r => r.name !== 'home').map(r => r.name) as never[];
    state.openKeys = matched
      .filter(r => r.path !== router.currentRoute.value.path)
      .map(r => r.name) as never[];
  },
  {
    immediate: true,
  },
);

// 路由转成菜单
const menuItems = computed(() => {
  const leftMenuRoutes = routes.find(o => o.name == 'home')!;
  const toMenuTree = (route: RouteRecordRaw) => {
    const meta = route.meta as unknown as RouterMeta;
    const childrens = route.children;
    let menu = {
      label: meta.title,
      key: route.name as string,
      children: childrens ? [] : null,
      onClick: () => router.push(route.path),
    };

    if (childrens) {
      for (const item of childrens) {
        const meta = item.meta as unknown as RouterMeta;
        if (meta.hide == true) {
          continue;
        }
        const m = toMenuTree(item);
        menu.children?.push(m as never);
      }
    }
    return menu;
  };
  return toMenuTree(leftMenuRoutes).children ?? [];
});
</script>
