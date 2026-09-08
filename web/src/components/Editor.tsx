type EditorProps = {
  path: string | null
  content: string
  isLoading: boolean
  saveMessage: string | null
  error: string | null
  onChange: (content: string) => void
  onSave: () => void
}

export function Editor({
  path,
  content,
  isLoading,
  saveMessage,
  error,
  onChange,
  onSave,
}: EditorProps) {
  return (
    <section className="editor-panel">
      <div className="editor-toolbar">
        <div>
          <span className="panel-label">Editor</span>
          <strong>{path ?? 'Select a file'}</strong>
        </div>
        <div className="editor-actions">
          {saveMessage && <span className="save-message">{saveMessage}</span>}
          {error && <span className="error-message">{error}</span>}
          <button
            type="button"
            className="save-button"
            disabled={!path || isLoading}
            onClick={onSave}
          >
            Save
          </button>
        </div>
      </div>
      <div className="editor-surface">
        {isLoading ? (
          <p className="empty-state">Loading file...</p>
        ) : (
          <textarea
            aria-label={path ? `Editing ${path}` : 'Editor'}
            disabled={!path}
            value={content}
            onChange={(event) => onChange(event.target.value)}
            spellCheck={false}
          />
        )}
      </div>
    </section>
  )
}
