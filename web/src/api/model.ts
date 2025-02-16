export interface PageInfo {
  page: number;
  pageSize: number;
}
export interface ApiResult<T = any> {
  code: number;
  message: string;
  data: T;
}
export interface ApiPageResult<T = any> {
  code: number;
  message: string;
  data: PageResult<T>;
}
export interface PageResult<T> {
  total: number;
  items: T[];
}

export interface SelectRows {
  value: number;
  label: string;
}
