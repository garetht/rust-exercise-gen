import {AnswerOption, SessionExercise} from "./models.ts";
import {Exercise} from "./protos/exercises.ts";

function shuffleArray<T>(a: T[]): T[] {
  return a.map(value => ({value, sort: Math.random()}))
      .sort((a, b) => a.sort - b.sort)
      .map(({value}) => value)
}

function chooseFromPool(pool: Map<string, Exercise[]>, totalNumber: number, minCompilable: number, maxCompilable: number) {
  // Structure
  // 1. Choose whether to have 1-3 compilable programs (out of 4)
  // 2. Then, choose a hard problem for the last one

  // 1. Choose whether to have 1-3 compilable programs (out of 4)
  const numCompilable = Math.floor(Math.random() * maxCompilable) + minCompilable;
  const numNonCompilable = totalNumber - numCompilable;

  const compilablePrograms = pool.get("") ?? [];
  const noncompilablePrograms = new Map(
      Array.from(pool).filter(([key]) => key !== "")
  );

  // choose three random keys from easyNoncompilablePrograms
  const noncompilableKeys = shuffleArray(Array.from(noncompilablePrograms.keys()))
      .slice(0, numNonCompilable);

  const compilableProgramsChosen = shuffleArray(compilablePrograms)
      .slice(0, numCompilable);
  const noncompilableProgramsChosen = noncompilableKeys
      .map((key) => (noncompilablePrograms.get(key) ?? [])[0])

  return shuffleArray(
      compilableProgramsChosen
          .map(e => createCompilableSessionExercise(e))
          .concat(noncompilableProgramsChosen
              .map(e => createNoncompilableSessionExercise(e))))
}

function createSession(easyPool: Map<string, Exercise[]>, hardPool: Map<string, Exercise[]>) {
  return chooseFromPool(easyPool, 3, 1, 2)
      .concat(chooseFromPool(hardPool, 2, 0, 1));
}

const possibleAnswers = [
  (s: string) => `borrow of moved value: \`${s}\``,
  (s: string) => `use of moved value: \`${s}\``,
  (s: string) => `cannot move out of \`${s}\` because it is borrowed`,
  (s: string) => `cannot borrow \`${s}\` as immutable because it is also borrowed as mutable`,
  // (s: string) => `cannot borrow \`${s}\` as mutable, as it is not declared as mutable`,
  // (s: string) => `cannot borrow \`${s}\` as mutable more than once at a time`,
  (s: string) => `cannot move out of \`*${s}\` which is behind a shared reference`,
  (s: string) => `cannot use \`${s}\` because it was mutably borrowed`,
]

function createCompilableSessionExercise(exercise: Exercise): SessionExercise {
  return {
    exercise: exercise,
    answerOptions: createCompilableAnswerOptions(exercise)
  }
}

function createCompilableAnswerOptions(exercise: Exercise): AnswerOption[] {
  let distractors = createDistractors(undefined, exercise.variableNames)
      .slice(0, 3)

  return [{
    text: <span><b>Yes</b>, it compiles!</span>,
    isCorrect: true
  }].concat(distractors);
}

function createNoncompilableSessionExercise(exercise: Exercise): SessionExercise {
  return {
    exercise: exercise,
    answerOptions: createNoncompilableAnswerOptions(exercise)
  }
}

function createNoncompilableAnswerOptions(exercise: Exercise) {
  let actualErrorMessage = exercise.errors[0].message
  let unshuffledAnswerOptions = createDistractors(actualErrorMessage, exercise.variableNames)
      .slice(0, 2)
      .concat({
        text: <span><b>No</b>, with the error: <pre className="error">{actualErrorMessage}</pre></span>,
        isCorrect: true
      })

  let answerOptions = shuffleArray(unshuffledAnswerOptions)

  return [{
    text: <span><b>Yes</b>, it compiles!</span>,
    isCorrect: false
  }].concat(answerOptions)
}

function createDistractors(actualErrorMessage: string | undefined, availableVariableNames: string[]): AnswerOption[] {
  let randomVariables = shuffleArray(availableVariableNames)

  let initialDistractors = [].concat(...Array(5).fill(randomVariables))
      .map((s) => possibleAnswers[Math.floor(Math.random() * possibleAnswers.length)](s))
      .filter((s) => s !== actualErrorMessage)

  let uniqueDistractors = new Set([...initialDistractors]);
  return [...uniqueDistractors].map((s) => {
    return {
      text: <span><b>No</b>, with the error: <pre className="error">{s}</pre></span>,
      isCorrect: false
    }
  })
}

export default createSession;
