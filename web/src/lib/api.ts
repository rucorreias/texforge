export type FileEntry = {
  name: string
  path: string
  type: 'file' | 'directory'
}

type FileListResponse = {
  entries: FileEntry[]
}

type FileResponse = {
  path: string
  content: string
}

function fileUrl(path: string) {
  return `/api/files/${path
    .split('/')
    .map((segment) => encodeURIComponent(segment))
    .join('/')}`
}

async function request<T>(url: string, options?: RequestInit): Promise<T> {
  const response = await fetch(url, options)
  if (!response.ok) {
    throw new Error(`Request failed (${response.status})`)
  }
  return response.json() as Promise<T>
}

export async function getFiles() {
  const response = await request<FileListResponse>('/api/files')
  return response.entries
}

export function getFile(path: string) {
  return request<FileResponse>(fileUrl(path))
}

export function saveFile(path: string, content: string) {
  return request<FileResponse>(fileUrl(path), {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ content }),
  })
}
