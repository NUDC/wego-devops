import { RouteRecordRaw } from 'vue-router';

const leftMenus: RouteRecordRaw[] = [
  {
    path: '/project/list',
    name: 'project_list',
    component: () => import('@/views/project/List.vue'),
    meta: { title: '项目列表', icon: 'icon-facebook' },
  },
  {
    path: '/project/save/:id(\\d+)?',
    name: 'projectsave',
    meta: { title: '项目详情', icon: 'icon-icon-test', hide: true },
    component: () => import('@/views/project/Save.vue'),
  },
];

export const ConstRoutes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'home',
    component: () => import('@/layouts/BasicLayout.vue'),
    meta: { title: '主页' },
    children: leftMenus,
    redirect: '/project/list',
  },
];
