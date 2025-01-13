import {EditorState} from '@codemirror/state'
import {EditorView, keymap} from '@codemirror/view'
import {defaultKeymap} from '@codemirror/commands'
import {rust} from '@codemirror/lang-rust'
import CodeMirror from '@uiw/react-codemirror';
import {lineNumbers} from '@codemirror/view'
import { bespin } from '@uiw/codemirror-themes-all';

interface CodeMirrorProps {
  initialDoc?: string,
  plainText: boolean,
  width: number
}

const CodeMirrorEditor = ({
                            initialDoc = "", plainText = false, width = 600
                          }: CodeMirrorProps) => {
  return <CodeMirror value={initialDoc.replace(/\n+$/, '').replace(/^\n+/, '').trim()}
                     style={{
                       height: 'auto',
                       width: `${width}px`,
                       overflow: 'auto',
                       border: '1px solid #333',
                       borderRadius: '8px',
                       fontSize: '16px',
                       textAlign: 'left',
                     }}
                     theme={bespin}
                     extensions={
                       [
                         keymap.of(defaultKeymap),
                         lineNumbers(),
                         EditorView.editable.of(false),
                         EditorState.readOnly.of(true),
                       ].concat(plainText ? [] : [rust()])
                     }/>
}
export default CodeMirrorEditor
