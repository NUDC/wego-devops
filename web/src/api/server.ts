import request from './VueAxios';

export interface ServerItem {
  ip: string;
  username: string;
  remark: string;
}

export function getServers() {
  return request.postJson<ServerItem[]>('/server/getall');
}
export function saveServer(data: ServerItem) {
  return request.postJson('/server/save', data);
}
export function delServers(ipList: string[]) {
  return request.postJson('/server/remove', ipList);
}
