export function getFileName(filepath: string) {
  const lastBackslashIndex = filepath.replace(/\\/g, '/').lastIndexOf('/');
  const fileName = filepath.slice(lastBackslashIndex + 1);
  return fileName;
}
