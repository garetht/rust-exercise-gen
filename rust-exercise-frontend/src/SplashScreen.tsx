import './App.css'

function SplashScreen({onStartExercises}: {onStartExercises: () => void}) {
  return (
    <>
      <div className="splash-title">Be the Borrow Checker</div>
      <div className="splash-text">
        Test your ability to find ownership errors in Rust programs!
      </div>
      <div className="splash-button" onClick={onStartExercises}>
        Start
      </div>
    </>
  )
}

export default SplashScreen;
