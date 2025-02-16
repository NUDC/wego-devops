import { ApiResult } from './model';
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
  buildScript: string;
  deployScript: string;
}

export interface ProjectDto {
  name: string;
}

export function getProjects() {
  return request.postJson<any, ApiResult<ProjectIndex[]>>('/getprojects', {});
}

export function delProject(data: string[]) {
  return request.postJson<string[], ApiResult>('/delproject', data);
}

export function getProjectInfo(name: string) {
  return request.postJson<ProjectDto, ApiResult<ProjectInfo>>('/getprojectinfo', { name });
}
export function setProjectInfo(data: ProjectInfo) {
  return request.postJson<ProjectInfo, ApiResult>('/setprojectinfo', data);
}

export function run(name: string) {
  return request.postJson<ProjectDto, ApiResult<string>>('/run', { name });
}
