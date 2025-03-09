import { notification } from 'ant-design-vue';
import axios, { type AxiosInstance, type AxiosRequestConfig } from 'axios';

export interface ApiResult<T = any> {
  code: number;
  message: string;
  data: T;
}

export interface VueAxiosInstance extends AxiosInstance {
  postJson<TResponse = any>(api: string, data?: any): Promise<ApiResult<TResponse>>;
}

export class VueAxios {
  private default: AxiosRequestConfig = {
    timeout: 6000,
    headers: {},
  };
  public request!: VueAxiosInstance;

  constructor(config: AxiosRequestConfig) {
    Object.assign(config, this.default);
    this.request = axios.create(config) as VueAxiosInstance;

    this.request.interceptors.response.use(o => o.data, this.errorHandler);

    this.request.postJson = <TResponse = any>(api: string, data?: any) => {
      return this.request.post<any, ApiResult<TResponse>>(api, JSON.stringify(data), {
        headers: {
          'Content-Type': 'application/json;charset=UTF-8',
        },
      });
    };
  }

  private errorHandler(error: any) {
    if (error.response) {
      notification.destroy();
      const data = error.response.data;
      if (error.response.status === 403) {
        notification.error({
          message: '无权限',
          description: data.message,
        });
      }
      if (error.response.status === 400) {
        notification.error({
          message: '请求参数错误',
          description: data.message,
        });
      }
      if (error.response.status === 500) {
        notification.error({
          message: '系统错误',
          description: data.message,
        });
      }

      if (error.response.status === 401) {
        notification.error({
          message: '未授权',
          description: data.message,
        });
      }
    }
    return Promise.reject(error);
  }
}

const host = import.meta.env.VITE_SERVER_HTTPAPI;
const request = new VueAxios({
  baseURL: `${host}`,
}).request;
export default request;
