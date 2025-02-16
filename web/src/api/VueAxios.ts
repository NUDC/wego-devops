import { notification } from 'ant-design-vue';
import axios, { type AxiosInstance, type AxiosRequestConfig } from 'axios';

export interface VueAxiosInstance extends AxiosInstance {
  getJson<TRequest, TResponse>(api: string, data: TRequest): Promise<TResponse>;
  postJson<TRequest, TResponse>(api: string, data: TRequest): Promise<TResponse>;
  deleteJson<TRequest, TResponse>(api: string, data: TRequest): Promise<TResponse>;
  putJson<TRequest, TResponse>(api: string, data: TRequest): Promise<TResponse>;
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

    this.request.getJson = <TRequest, TResponse>(api: string, data: TRequest) => {
      return this.request.get<TRequest, TResponse>(api, { params: data });
    };

    this.request.postJson = <TRequest, TResponse>(api: string, data: TRequest) => {
      return this.request.post<TRequest, TResponse>(api, JSON.stringify(data), {
        headers: {
          'Content-Type': 'application/json;charset=UTF-8',
        },
      });
    };

    this.request.putJson = <TRequest, TResponse>(api: string, data: TRequest) => {
      return this.request.put<TRequest, TResponse>(api, JSON.stringify(data), {
        headers: {
          'Content-Type': 'application/json;charset=UTF-8',
        },
      });
    };

    this.request.deleteJson = <TRequest, TResponse>(api: string, data: TRequest) => {
      return this.request.delete<TRequest, TResponse>(api, {
        data: JSON.stringify(data),
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
