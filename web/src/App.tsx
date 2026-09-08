import { useEffect, useState } from 'react'
import './App.css'
import { Editor } from './components/Editor'
import { FileExplorer } from './components/FileExplorer'
import { getFile, getFiles, saveFile, type FileEntry } from './lib/api'

function App() {
  const [entries, setEntries] = useState<FileEntry[]>([])
  const [selectedPath, setSelectedPath] = useState<string | null>(null)
  const [content, setContent] = useState('')
  const [isLoading, setIsLoading] = useState(true)
  const [isLoadingFile, setIsLoadingFile] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [saveMessage, setSaveMessage] = useState<string | null>(null)

  useEffect(() => {
    async function loadFiles() {
      try {
        const files = await getFiles()
        setEntries(files)
        const firstFile = files.find((entry) => entry.type === 'file')
        if (firstFile) await selectFile(firstFile.path)
      } catch (loadError) {
        setError(loadError instanceof Error ? loadError.message : 'Failed to load files')
      } finally {
        setIsLoading(false)
      }
    }

    void loadFiles()
  }, [])

  async function selectFile(path: string) {
    setSelectedPath(path)
    setIsLoadingFile(true)
    setSaveMessage(null)
    setError(null)

    try {
      const file = await getFile(path)
      setContent(file.content)
    } catch (loadError) {
      setError(loadError instanceof Error ? loadError.message : 'Failed to load file')
      setContent('')
    } finally {
      setIsLoadingFile(false)
    }
  }

  async function handleSave() {
    if (!selectedPath) return

    setSaveMessage(null)
    setError(null)
    try {
      await saveFile(selectedPath, content)
      setSaveMessage('Saved')
    } catch (saveError) {
      setError(saveError instanceof Error ? saveError.message : 'Failed to save file')
    }
  }

  return (
    <main className="ide-shell">
      <header className="topbar">
        <div>
          <h1>TexForge</h1>
        </div>
      </header>

      <div className="workspace">
        <FileExplorer
          entries={entries}
          selectedPath={selectedPath}
          isLoading={isLoading}
          onSelect={selectFile}
        />
        <Editor
          path={selectedPath}
          content={content}
          isLoading={isLoadingFile}
          saveMessage={saveMessage}
          error={error}
          onChange={setContent}
          onSave={handleSave}
        />
      </div>
    </main>
  )
}

export default App
