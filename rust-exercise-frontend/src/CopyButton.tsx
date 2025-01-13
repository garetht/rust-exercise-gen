import './App.css'
import {useState} from "react";

function CopyButton({ text, className }: {text: string, className: string}) {
  const [status, setStatus] = useState('idle');

  const copyToClipboard = async () => {
    try {
      await navigator.clipboard.writeText(text);
      setStatus('copied');
      setTimeout(() => setStatus('idle'), 2500);
    } catch (err) {
      console.error('Failed to copy:', err);
      setStatus('error');
    }
  };

  return (
      <button
          onClick={copyToClipboard}
          className={className}
          disabled={status === 'copied'}
      >
        {status === 'idle' && 'Share Results'}
        {status === 'copied' && '✓ Copied!'}
        {status === 'error' && '❌ Failed to copy'}
      </button>
  );
}

export default CopyButton;
