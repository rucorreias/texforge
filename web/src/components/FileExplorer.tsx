import type { FileEntry } from '../lib/api'

type FileExplorerProps = {
  entries: FileEntry[]
  selectedPath: string | null
  isLoading: boolean
  onSelect: (path: string) => void
}

export function FileExplorer({
  entries,
  selectedPath,
  isLoading,
  onSelect,
}: FileExplorerProps) {
  return (
    <aside className="file-explorer">
      <div className="panel-heading">
        <span className="panel-label">Project files</span>
        <span className="file-count">{entries.length}</span>
      </div>
      <div className="file-list" aria-label="Project files">
        {isLoading && <p className="empty-state">Loading files...</p>}
        {!isLoading && entries.length === 0 && (
          <p className="empty-state">No files found</p>
        )}
        {entries.map((entry) => {
          const depth = entry.path.split('/').length - 1
          const isSelected = entry.path === selectedPath

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

          return (
            <button
              className={`file-row file-row-button${isSelected ? ' selected' : ''}`}
              key={entry.path}
              type="button"
              onClick={() => onSelect(entry.path)}
              style={{ paddingLeft: `${16 + depth * 16}px` }}
            >
              <span className="entry-icon file-icon" aria-hidden="true">T</span>
              <span>{entry.name}</span>
            </button>
          )
        })}
      </div>
    </aside>
  )
}
