import request from './VueAxios';

export interface ProjectIndex {
  name: string;
  remark: string;
  status: number;
  buildTime: string;
  created: string;
}
export interface AddProjectParams {
  name: string;
  remark: string;
}
export interface ProjectInfo {
  name: string;
  remark: string;
  group?: string;
  buildScript: string;
  deploy: {
    ip: string;
    deployScript: string;
  }[];
}

export interface ProjectUniqueId {
  name: string;
  group?: string;
}

export interface ProjectDepolyDto extends ProjectUniqueId {
  ip?: string;
}
export interface ProjectLog {
  path: string;
}

export function getProjects() {
  return request.postJson<ProjectIndex[]>('/project/getall');
}
export function delProject(data: ProjectUniqueId[]) {
  return request.postJson('/project/remove', data);
}

export function getProjectInfo(data: ProjectUniqueId) {
  return request.postJson<ProjectInfo>('/project/getinfo', data);
}
export function setProjectInfo(data: ProjectInfo) {
  return request.postJson('/project/save', data);
}
export function build(data: ProjectUniqueId) {
  return request.postJson('/project/build', data);
}
export function deploy(data: ProjectDepolyDto) {
  return request.postJson('/project/deploy', data);
}
export function run(data: ProjectDepolyDto) {
  return request.postJson('/project/run', data);
}

export function getLogs(data: ProjectUniqueId) {
  return request.postJson<string[]>('/project/getlogs', data);
}
export function getLog(path: string) {
  return request.postJson<string>('/project/getlog', { path });
}
