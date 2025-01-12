import {useEffect, useRef} from 'react'
import {EditorState} from '@codemirror/state'
import {EditorView, keymap} from '@codemirror/view'
import {defaultKeymap} from '@codemirror/commands'
import {rust} from '@codemirror/lang-rust'
import {oneDark} from '@codemirror/theme-one-dark'
import {lineNumbers} from '@codemirror/view'
import {syntaxHighlighting, defaultHighlightStyle} from '@codemirror/language'

interface CodeMirrorProps {
  initialDoc?: string,
  onChange?: (state: EditorState) => void
}

const CodeMirrorEditor = ({
                            initialDoc = "", onChange = () => {
  }
                          }: CodeMirrorProps) => {
  const editor = useRef<HTMLDivElement>(null)

  useEffect(() => {
    if (!editor.current) return

    const state = EditorState.create({
      doc: initialDoc,
      extensions: [
        keymap.of(defaultKeymap),
        rust(),
        oneDark,
        lineNumbers(),
        syntaxHighlighting(defaultHighlightStyle),
        EditorView.editable.of(false),
        EditorState.readOnly.of(true),
        EditorState.tabSize.of(4),
        EditorView.updateListener.of((update) => {
          if (update.changes) {
            onChange(update.state)
          }
        })
      ]
    })

    const view = new EditorView({
      state,
      parent: editor.current
    })

    return () => {
      view.destroy()
    }
  }, [editor.current])

  return <div ref={editor} style={{
    height: 'auto',
    width: '500px',
    overflow: 'auto',
    border: '1px solid #333',
    textAlign: 'left'
  }}/>
}

export default CodeMirrorEditor
