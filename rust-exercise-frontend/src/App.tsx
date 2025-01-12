import './App.css'
import exerciseProtos from './assets/exercises.pb.bin?uint8array';
import {Exercises} from "./protos/exercises.ts";
import CodeMirrorEditor from "./CodeMirrorEditor.tsx";

function App() {
  let exercises = Exercises.decode(exerciseProtos);
  return (
    <>
      <CodeMirrorEditor initialDoc={exercises.exerciseGroups[0].exercises[0].formattedProgram}/>
    </>
  )
}

export default App
