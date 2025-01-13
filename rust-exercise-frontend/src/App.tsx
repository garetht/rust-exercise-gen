import React from "react";
import {SwitchTransition, CSSTransition} from "react-transition-group";
import "./App.css";
import exerciseProtos from './assets/exercises.pb.bin?uint8array';
import {Exercise, Exercises} from "./protos/exercises.ts";
import SplashScreen from "./SplashScreen.tsx";
import ExerciseScreen from "./ExerciseScreen.tsx";
import createSession from "./SessionCreator.tsx";
import {ExerciseState, SessionExercise} from "./models.ts";
import ShareScreen from "./ShareScreen.tsx";

let exercises = Exercises.decode(exerciseProtos);

function filterExercisePool(exercises: Exercises, f: (e: Exercise) => boolean): Map<string, Exercise[]> {
  return exercises.exerciseGroups
      .filter(group => group.errorCodes.length <= 1)
      .reduce((acc, next) => {
        let filteredExercises = next
            .exercises
            .filter(f);
        if (filteredExercises.length > 0) {
          acc.set(next.errorCodes.length === 0 ? "" : next.errorCodes[0], filteredExercises)
        }
        return acc
      }, new Map<string, Exercise[]>())
}

let exerciseEasyPoolByCode = filterExercisePool(
    exercises,
    (exercise) => exercise.errors.length <= 1 && exercise.programLength <= 3)

let exerciseHardPoolByCodes = filterExercisePool(
    exercises,
    (exercise) => exercise.errors.length <= 1 && exercise.programLength > 3)

export default function App() {
  const [state, setState] = React.useState<ExerciseState>('splash');
  const [session, setSession] = React.useState<SessionExercise[]>([]);
  const [sessionIndex, setSessionIndex] = React.useState<number>(0);
  const splashRef = React.useRef(null);
  const inProgressRef = React.useRef(null);
  const nodeRef = state === 'splash' ? splashRef : inProgressRef;
  const onStart = () => {
    setState('in-progress');
    setCorrectIndices([]);
    setSession(createSession(exerciseEasyPoolByCode, exerciseHardPoolByCodes));
    setSessionIndex(0);
  }
  const [correctIndices, setCorrectIndices] = React.useState<number[]>([]);
  const onAdvance = (wasCorrect: boolean) => {
    if (wasCorrect) {
      setCorrectIndices(correctIndices.concat(sessionIndex));
    }
    if (sessionIndex < session.length - 1) {
      setSessionIndex(sessionIndex + 1);
    } else {
      setState('completed');
    }
  }

  return (
      <>
        <div className="main">
          <SwitchTransition mode="out-in">
            <CSSTransition
                key={state}
                nodeRef={nodeRef}
                addEndListener={(done) => {
                  nodeRef.current.addEventListener("transitionend", done, false);
                }}
                classNames="fade"
            >
              <div ref={nodeRef}>
                {(() => {
                  switch (state) {
                    case 'splash':
                      return <SplashScreen onStartExercises={onStart}/>;
                    case 'in-progress':
                      return <ExerciseScreen
                          onAdvance={onAdvance}
                          correctIndices={correctIndices}
                          currentIndex={sessionIndex}
                          totalLength={session.length}
                          currentExercise={session[sessionIndex]}
                      />;
                    case 'completed':
                      return <ShareScreen
                          correctIndices={correctIndices}
                          sessionLength={session.length}
                          onRestart={onStart}/>;
                    default:
                      return null;
                  }
                })()}
              </div>
            </CSSTransition>
          </SwitchTransition>
        </div>
      </>
  );
}
