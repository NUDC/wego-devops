import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/reset.css';

import App from './App.vue';
import store from './store';
import { router } from './router';
import VueSocket from './api/VueSocket';

createApp(App).use(store).use(router).use(Antd).use(VueSocket).mount('#app');
