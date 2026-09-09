import { useEffect, useRef, useState } from 'react'
import './App.css'
import { Editor } from './components/Editor'
import { FileExplorer } from './components/FileExplorer'
import { MenuBar } from './components/MenuBar'
import { getFile, getFiles, saveFile, type FileEntry } from './lib/api'

function App() {
  const [entries, setEntries] = useState<FileEntry[]>([])
  const [selectedPath, setSelectedPath] = useState<string | null>(null)
  const [content, setContent] = useState('')
  const [isLoading, setIsLoading] = useState(true)
  const [isLoadingFile, setIsLoadingFile] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [saveMessage, setSaveMessage] = useState<string | null>(null)
  const [showGenerated, setShowGenerated] = useState(false)
  const [compactLayout, setCompactLayout] = useState(false)
  const textareaRef = useRef<HTMLTextAreaElement | null>(null)

  useEffect(() => {
    void loadFiles(true)
  }, [])

  async function loadFiles(selectFirstFile = false) {
    setIsLoading(true)
    setError(null)
    try {
      const files = await getFiles()
      setEntries(files)
      if (selectFirstFile) {
        const firstFile = files.find((entry) => entry.type === 'file')
        if (firstFile) await selectFile(firstFile.path)
      } else if (selectedPath && files.some((entry) => entry.path === selectedPath)) {
        await selectFile(selectedPath)
      }
    } catch (loadError) {
      setError(loadError instanceof Error ? loadError.message : 'Failed to load files')
    } finally {
      setIsLoading(false)
    }
  }

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

  function handleUndo() {
    textareaRef.current?.focus()
    document.execCommand('undo')
  }

  function handleRedo() {
    textareaRef.current?.focus()
    document.execCommand('redo')
  }

  function handleSelectAll() {
    textareaRef.current?.focus()
    textareaRef.current?.select()
  }

  return (
    <main className={`ide-shell${compactLayout ? ' compact-layout' : ''}`}>
      <header className="topbar">
        <MenuBar
          canSave={Boolean(selectedPath) && !isLoadingFile}
          showGenerated={showGenerated}
          compactLayout={compactLayout}
          onSave={handleSave}
          onReload={() => void loadFiles()}
          onUndo={handleUndo}
          onRedo={handleRedo}
          onSelectAll={handleSelectAll}
          onShowGeneratedChange={setShowGenerated}
          onCompactLayoutChange={setCompactLayout}
        />
      </header>

      <div className="workspace">
        <FileExplorer
          entries={entries}
          selectedPath={selectedPath}
          isLoading={isLoading}
          showGenerated={showGenerated}
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
          textareaRef={textareaRef}
        />
      </div>
    </main>
  )
}

export default App
