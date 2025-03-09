import { RouteRecordRaw } from 'vue-router';

const leftMenus: RouteRecordRaw[] = [
  {
    path: '/project/list',
    name: 'project_list',
    component: () => import('@/views/project/List.vue'),
    meta: { title: '项目', icon: 'icon-facebook' },
  },
  {
    path: '/server/list',
    name: 'server_list',
    component: () => import('@/views/server/List.vue'),
    meta: { title: '服务器', icon: 'icon-facebook' },
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
