import './App.css'
import CodeMirrorEditor from "./CodeMirrorEditor.tsx";
import {SessionExercise} from "./models.ts";
import React, {useState} from "react";
import {CSSTransition, SwitchTransition} from "react-transition-group";

type ExerciseState = 'show-exercise' | 'show-grading';

function determineIndicatorClass(index: number, currentIndex: number, correctIndices: number[]) {
  let indicatorClass = "";
  if (currentIndex === index) {
    indicatorClass = "current";
  } else if (correctIndices.includes(index)) {
    indicatorClass = "correct";
  } else if (!correctIndices.includes(index) && currentIndex > index) {
    indicatorClass = "failed";
  }
  return indicatorClass;
}

function ExerciseScreen({currentExercise, correctIndices, currentIndex, totalLength, onAdvance}: {
  currentExercise: SessionExercise,
  currentIndex: number,
  correctIndices: number[],
  totalLength: number,
  onAdvance: (wasCorrect: boolean) => void
}) {
  let [state, setState] = useState<ExerciseState>('show-exercise');
  let [correctness, setCorrectness] = useState<boolean>(false);
  const exerciseRef = React.useRef(null);
  const gradingRef = React.useRef(null);
  const nodeRef = state === 'show-exercise' ? exerciseRef : gradingRef;

  return (
      <div className={"exercise-screen"}>
        <div className="exercise-indicators">
          {
            Array.from({length: totalLength}).map((_, index) =>
                <div key={index} className={"exercise-indicator " + determineIndicatorClass(index, currentIndex, correctIndices)}/>
            )
          }
        </div>
        <CodeMirrorEditor initialDoc={currentExercise.exercise.formattedProgram} plainText={false} width={600}/>
        <SwitchTransition mode="out-in">
          <CSSTransition
              key={state}
              nodeRef={nodeRef}
              // @ts-ignore
              addEndListener={(done) => {
                // @ts-ignore
                nodeRef.current.addEventListener("transitionend", done, false);
              }}
              classNames="fade"
          >
            <div ref={nodeRef}>
              {(() => {
                switch (state) {
                  case 'show-exercise':
                    return renderAnswerOptions(
                        currentExercise,
                        (isCorrect) => {
                          setState('show-grading');
                          setCorrectness(isCorrect);
                        }
                    );
                  case 'show-grading':
                    return renderGrading(correctness, currentExercise.exercise.humanErrors, () => {
                      setState('show-exercise');
                      onAdvance(correctness);
                    })
                  default:
                    return null;
                }
              })()}
            </div>
          </CSSTransition>
        </SwitchTransition>
      </div>
  )
}

function renderAnswerOptions(currentExercise: SessionExercise,
                             onGrade: (isCorrect: boolean) => void): React.ReactElement {
  return <>
    <div className={"question"}>
      Does this program compile?
    </div>
    <div className="options">
      {
        currentExercise.answerOptions.map((option, index) =>
            <div className="option-button" key={index} onClick={() => onGrade(option.isCorrect)}>
              {option.text}
            </div>
        )
      }
    </div>
  </>
}

function renderGrading(isCorrect: boolean, humanErrors: string[], onAdvance: () => void) {
  let noErrors = <div>
    The program satisfies the borrow checker and compiles without any errors.
  </div>;
  let hasErrors = humanErrors.length === 1 ? <div className="explanation-text">
    <p>The program does not satisfy the borrow checker and the compiler produces this error:</p>
    <CodeMirrorEditor initialDoc={humanErrors[0]} plainText={true} width={544}></CodeMirrorEditor>
  </div> : <div className="explanation-text">
    <p>The program does not satisfy the borrow checker and the compiler produces these errors:</p>
    {
      humanErrors.map(error => {
        return <CodeMirrorEditor initialDoc={error} plainText={true} width={544}></CodeMirrorEditor>
      })
    }
  </div>;

  return <>
    <div className="grading">
      {
        isCorrect ? <div className="correct-grading">
          Correct!
        </div> : <div className="incorrect-grading">
          Incorrect!
        </div>
      }
      {
        humanErrors.length === 0 ? noErrors : hasErrors
      }
    </div>
    <div className="option-button" onClick={onAdvance}>Continue</div>
  </>;
}

export default ExerciseScreen;
