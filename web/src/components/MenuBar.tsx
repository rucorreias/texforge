import { useEffect, useRef, useState, type ReactNode } from 'react'

type MenuName = 'file' | 'edit' | 'view' | null

type MenuBarProps = {
  canSave: boolean
  showGenerated: boolean
  compactLayout: boolean
  onSave: () => void
  onReload: () => void
  onUndo: () => void
  onRedo: () => void
  onSelectAll: () => void
  onShowGeneratedChange: (show: boolean) => void
  onCompactLayoutChange: (compact: boolean) => void
}

export function MenuBar({
  canSave,
  showGenerated,
  compactLayout,
  onSave,
  onReload,
  onUndo,
  onRedo,
  onSelectAll,
  onShowGeneratedChange,
  onCompactLayoutChange,
}: MenuBarProps) {
  const [openMenu, setOpenMenu] = useState<MenuName>(null)
  const menuRef = useRef<HTMLElement>(null)

  useEffect(() => {
    function closeOnOutsideClick(event: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setOpenMenu(null)
      }
    }

    document.addEventListener('mousedown', closeOnOutsideClick)
    return () => document.removeEventListener('mousedown', closeOnOutsideClick)
  }, [])

  function toggleMenu(menu: Exclude<MenuName, null>) {
    setOpenMenu((current) => (current === menu ? null : menu))
  }

  function run(action: () => void) {
    action()
    setOpenMenu(null)
  }

  return (
    <nav className="menu-bar" ref={menuRef} aria-label="Application menu">
      <div className="brand-mark">TexForge</div>
      <div className="menu-items">
        <MenuButton label="File" isOpen={openMenu === 'file'} onClick={() => toggleMenu('file')}>
          <MenuItem label="Save" shortcut="Cmd S" disabled={!canSave} onClick={() => run(onSave)} />
          <MenuItem label="Reload project" onClick={() => run(onReload)} />
        </MenuButton>
        <MenuButton label="Edit" isOpen={openMenu === 'edit'} onClick={() => toggleMenu('edit')}>
          <MenuItem label="Undo" shortcut="Cmd Z" disabled={!canSave} onClick={() => run(onUndo)} />
          <MenuItem label="Redo" shortcut="Shift Cmd Z" disabled={!canSave} onClick={() => run(onRedo)} />
          <MenuItem label="Select all" shortcut="Cmd A" disabled={!canSave} onClick={() => run(onSelectAll)} />
        </MenuButton>
        <MenuButton label="View" isOpen={openMenu === 'view'} onClick={() => toggleMenu('view')}>
          <MenuCheckbox
            label="Show generated files"
            checked={showGenerated}
            onChange={(checked) => {
              onShowGeneratedChange(checked)
              setOpenMenu(null)
            }}
          />
          <MenuCheckbox
            label="Compact layout"
            checked={compactLayout}
            onChange={(checked) => {
              onCompactLayoutChange(checked)
              setOpenMenu(null)
            }}
          />
        </MenuButton>
      </div>
    </nav>
  )
}

type MenuButtonProps = {
  label: string
  isOpen: boolean
  onClick: () => void
  children: ReactNode
}

function MenuButton({ label, isOpen, onClick, children }: MenuButtonProps) {
  return (
    <div className="menu-item">
      <button className={`menu-trigger${isOpen ? ' active' : ''}`} type="button" aria-expanded={isOpen} onClick={onClick}>
        {label}
      </button>
      {isOpen && <div className="menu-dropdown">{children}</div>}
    </div>
  )
}

type MenuItemProps = {
  label: string
  shortcut?: string
  disabled?: boolean
  onClick: () => void
}

function MenuItem({ label, shortcut, disabled = false, onClick }: MenuItemProps) {
  return (
    <button className="menu-command" type="button" disabled={disabled} onClick={onClick}>
      <span>{label}</span>
      {shortcut && <span className="menu-shortcut">{shortcut}</span>}
    </button>
  )
}

type MenuCheckboxProps = {
  label: string
  checked: boolean
  onChange: (checked: boolean) => void
}

function MenuCheckbox({ label, checked, onChange }: MenuCheckboxProps) {
  return (
    <label className="menu-command menu-checkbox">
      <span>{label}</span>
      <input type="checkbox" checked={checked} onChange={(event) => onChange(event.target.checked)} />
    </label>
  )
}
