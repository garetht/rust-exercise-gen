import {Exercise} from "./protos/exercises.ts";
import React from "react";

export type SessionExercise = {
  exercise: Exercise,
  answerOptions: AnswerOption[]
}

export type AnswerOption = {
  text: React.ReactElement,
  isCorrect: boolean
}
export type ExerciseState = 'splash' | 'in-progress' | 'completed';

