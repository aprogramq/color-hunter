import { useContext, useEffect, useState, type Dispatch, type SetStateAction } from 'react'
import type { BaseHexColor } from '../hex'
import { type displayT, type sizeT, type UseState, type paletteT, options } from '../types'
import { SizeContext } from '..'
import { useKeyboardMain } from '../keyboard'
import { Modal } from './Modal'
import { Palette } from './start/Colors'
import { randomColor } from '../fuctions'
import { Header } from './start/Header'

export default function Main({
  actionOptions,
  selectedIndex,
  setSelectedIndex,
}: {
  actionOptions: (option: displayT) => void
  selectedIndex: number
  setSelectedIndex: UseState<number>
}) {
  const [palette, setPalette] = useState<paletteT>({
    colors: [[]] as BaseHexColor[][],
    size: 3,
    display: false,
    position: 0,
  })
  const [pause, setPause] = useState<boolean>(true)
  const size = useContext<sizeT>(SizeContext)
  const displayLogo = size.height > 13 || !palette.display
  const [displayModal, setDisplayModal] = useState<boolean>(false)
  const [timeout, setMyTimeout] = useState<NodeJS.Timeout>()

  useEffect(() => {
    setMyTimeout(randomColor(palette, setPalette, options[selectedIndex]!.value))
    setPalette((p) => ({ ...p, display: true }))
    setPause(false)
  }, [])

  const states = {
    palette,
    setPalette,
    selectedIndex,
    setSelectedIndex,
    pause,
    setPause,
    actionOptions,
    setDisplayModal,
    timeout,
    setMyTimeout,
  }

  useKeyboardMain(states)
  return (
    <box alignItems="center" justifyContent="center" flexGrow={1} marginTop={1}>
      <Header activate={displayLogo} />
      <Palette colorsPalette={palette.colors} position={palette.position} count={palette.size} />

      {pause && (
        <>
          <text
            fg="#ffffff"
            position="absolute"
            right={size.width / 1.4}
            top={size.height / 1.9}
          >{`${palette.position === 1 ? '' : '<-- [B]ack'}`}</text>
          <text
            fg="#ffffff"
            position="absolute"
            left={size.width / 1.4}
            top={size.height / 1.9}
          >{`${palette.position === palette.colors.length - 1 ? '' : '[N]ext -->'}`}</text>

          <box flexDirection="row" position="absolute" top={size.height / 1.6}>
            <box padding={1} paddingLeft={4} paddingRight={4} marginTop={1}>
              <text fg="#ffffff">[C]ontinue</text>
            </box>
            <box padding={1} paddingLeft={4} paddingRight={4} marginTop={1}>
              <text fg="#ffffff">[S]ave palette</text>
            </box>
          </box>
        </>
      )}
      {!pause  && (
        <>
          <text fg="#ffffff" position="absolute" top={size.height / 1.5}>
            Press [space] to stop
          </text>
        </>
      )}
      <box padding={1}>
        <box top={size.height - 2} left={5} position="absolute" width={20}>
          <text fg="#ffffff" style={{ marginBottom: 1 }}>
            [E]xit
          </text>
        </box>
      </box>
        <Modal activate={displayModal} />
    </box>
  )
}
