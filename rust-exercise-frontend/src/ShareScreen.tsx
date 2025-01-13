import './App.css'
import CopyButton from "./CopyButton.tsx";

function ShareScreen({onRestart, correctIndices, sessionLength}: {
  onRestart: () => void,
  correctIndices: number[],
  sessionLength: number
}) {
  const indiceSet = new Set(correctIndices);
  const shareText = `
I got ${correctIndices.length} out of ${sessionLength} at Be the Borrow Checker!

${
      Array.from({
        length: sessionLength
      }, (_, i) => {
        return indiceSet.has(i) ? String.fromCodePoint(0x1F7E9) : String.fromCodePoint(0x1F7E5)
      })
          .join('')
  }

See if you can beat my score: ${window.location.href}`;

  return (
      <div className="share-screen">
        <div className="result">
          You got {correctIndices.length} out of {sessionLength} right!
        </div>
        <CopyButton text={shareText} className="option-button"/>
        <div className="option-button" onClick={onRestart}>
          Restart
        </div>
      </div>
  )
}

export default ShareScreen;
