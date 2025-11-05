import { render, useKeyboard, useRenderer } from '@opentui/react'
import { createContext, useEffect, useState } from 'react'
import { type displayT, type sizeT } from './types'
import { StartScreen } from './components/Start'
import { Options } from './components/Options'
import { initialize } from './init'
import Main from './components/Main'
import { useKeyboardStart } from './keyboard'

export const SizeContext = createContext<sizeT>({ height: 0, width: 0 })

function App() {
  initialize()
  const [display, setDisplay] = useState<displayT>('start')
  const render = useRenderer()
  const size = { height: render.height, width: render.width }
  const [selectedIndex, setSelectedIndex] = useState<number>(0)
  useEffect(() => {
    render.console.show()
  })

  const actionOptions = (display: displayT) => {
    switch (display) {
      case 'start':
        setDisplay('start')
        break
      case 'options':
        setDisplay('options')
        break
      case 'main':
        setDisplay('main')
        break
    }
  }
  return (
    <>
      <SizeContext value={size}>
        {display === 'start' && <StartScreen setSelectedIndex={setSelectedIndex} actionOptions={actionOptions} />}
		{display === 'main' && <Main actionOptions={actionOptions} selectedIndex={selectedIndex} setSelectedIndex={setSelectedIndex}/>}
        {display === 'options' && <Options actionOptions={actionOptions} />}
      </SizeContext>
    </>
  )
}

render(<App />)
