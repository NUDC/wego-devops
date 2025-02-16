import { createRouter, createWebHistory } from 'vue-router';
import { ConstRoutes } from './const-routes';

const router = createRouter({
  history: createWebHistory(),
  routes: ConstRoutes,
});

export { router };
