import { render, useRenderer } from '@opentui/react'
import { createContext, useState } from 'react'
import { type displayT, type sizeT } from './types'
import { StartScreen } from './components/start/Start'
import { Options } from './components/Options'
import { initialize } from './init'

export const SizeContext = createContext<sizeT>({ height: 0, width: 0 })

function App() {
	initialize()
	const [display, setDisplay] = useState<displayT>('start')
	const render = useRenderer()
	// useEffect(()=>{
	// 	render.console.show()
	// })
	const size = { height: render.height, width: render.width }

	const enterOptions = () => {
		setDisplay('options')
	}
	const exitOptions = () => {
		setDisplay('start')
	}
	return (
		<>
			<SizeContext value={size}>
				<StartScreen enterOptions={enterOptions} isActive={display === 'start'} />
				<Options exitOptions={exitOptions} isActive={display === 'options'} />
			</SizeContext>
		</>
	)
}

render(<App />)
