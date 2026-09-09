import type { FileEntry } from '../lib/api'

type FileExplorerProps = {
  entries: FileEntry[]
  selectedPath: string | null
  isLoading: boolean
  showGenerated: boolean
  onSelect: (path: string, readOnly: boolean) => void
}

const generatedExtensions = [
  '.aux',
  '.bbl',
  '.bcf',
  '.blg',
  '.fdb_latexmk',
  '.fls',
  '.lof',
  '.log',
  '.lot',
  '.out',
  '.pdf',
  '.run.xml',
  '.synctex.gz',
  '.toc',
]

export function isGeneratedFile(path: string) {
  return generatedExtensions.some((extension) => path.endsWith(extension))
}

export function FileExplorer({
  entries,
  selectedPath,
  isLoading,
  showGenerated,
  onSelect,
}: FileExplorerProps) {
  const visibleEntries = entries.filter(
    (entry) => entry.type === 'directory' || showGenerated || !isGeneratedFile(entry.path),
  )
  const visibleFileCount = visibleEntries.filter((entry) => entry.type === 'file').length

  return (
    <aside className="file-explorer">
      <div className="panel-heading">
        <div>
          <span className="panel-label">Project files</span>
          <span className="file-count">{visibleFileCount}</span>
        </div>
      </div>
      <div className="file-list" aria-label="Project files">
        {isLoading && <p className="empty-state">Loading files...</p>}
        {!isLoading && visibleEntries.length === 0 && (
          <p className="empty-state">
            {showGenerated ? 'No files found' : 'No editable files found'}
          </p>
        )}
        {visibleEntries.map((entry) => {
          const depth = entry.path.split('/').length - 1
          const isSelected = entry.path === selectedPath
          const generated = isGeneratedFile(entry.path)

          if (entry.type === 'directory') {
            return (
              <div
                className="file-row directory-row"
                key={entry.path}
                style={{ paddingLeft: `${16 + depth * 16}px` }}
              >
                <span className="entry-icon" aria-hidden="true">+</span>
                <span>{entry.name}</span>
              </div>
            )
          }

          const className = `file-row file-row-button${isSelected ? ' selected' : ''}${generated ? ' generated-row' : ''}`
          const fileContent = (
            <>
              <span className="entry-icon file-icon" aria-hidden="true">T</span>
              <span>{entry.name}</span>
            </>
          )

          return (
            <button
              className={className}
              key={entry.path}
              type="button"
              onClick={() => onSelect(entry.path, generated)}
              style={{ paddingLeft: `${16 + depth * 16}px` }}
            >
              {fileContent}
            </button>
          )
        })}
      </div>
    </aside>
  )
}
