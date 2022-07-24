import { useEffect, useState } from 'react';
import './App.css';

import { Interpreter } from 'stackgrid';

/**
 * @type {string[][]}
 */
const ip = [];

function App() {
  const [input, setInput] = useState(ip);
  const [debugs, setDebugs] = useState([]);

  useEffect(() => {
    onReset();
  }, []);

  const onInput = (rowIndex, columnIndex, value) => {
    const nextInput = [...input];
    nextInput[rowIndex][columnIndex] = value;
    setInput(nextInput);
  };

  const onReset = () => {
    const query = new URLSearchParams(window.location.search).get('q');
    if (query) {
      const queryInput = query.split('\n').map((row) => row.split('\t'));
      setInput(() => [...queryInput]);
    } else {
      setInput(() => [...Array.from(new Array(10), () => Array.from(new Array(5), () => ''))]);
    }
  };

  const collectDebug = (state) => {
    const deepCopy = state.map((row) => [...row.map((cell) => ({ ...cell }))]);
    setDebugs((debugs) => [...debugs, deepCopy]);
  };

  const onRun = () => {
    const rows = input.map((inputRow, rowIndex) => ({
      index: rowIndex,
      cells: inputRow.map((inputCell, colIndex) => ({
        index: colIndex,
        rowIndex: rowIndex,
        instruction: {
          value: inputCell,
        },
      })),
    }));

    setDebugs(() => []);

    const interpreter = new Interpreter(rows, null, console.log, collectDebug);
    interpreter.interpret();
  };

  const onDebug = () => {
    const str = input.map((row) => row.map((cell) => cell).join('\t')).join('\n');
    const url = encodeURIComponent(str);
    console.log(url);
  };

  const onReplay = () => {
    console.log(debugs[0]);
    let i = 0;
    const interval = setInterval(() => {
      if (i++ >= debugs.length - 1) {
        clearInterval(interval);
        return;
      }

      setInput(() => {
        const newInput = [...debugs[i].map((row) => row.map((cell) => cell.value))];
        return newInput;
      });
    }, 500);
  };

  return (
    <div className="App">
      <table>
        <tbody>
          {input.map((row, rowIndex) => (
            <tr>
              {row.map((cell, colIndex) => (
                <td>
                  <input
                    type="text"
                    onChange={(evt) => onInput(rowIndex, colIndex, evt.target.value)}
                    value={cell ? cell : ''}
                  />
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
      <button onClick={onRun}>Run</button>
      <button onClick={onReset}>Reset</button>
      <button onClick={onDebug}>Debug</button>
      <button onClick={onReplay}>Replay</button>
    </div>
  );
}

export default App;
