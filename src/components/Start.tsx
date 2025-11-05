import { useContext, useState, type Dispatch } from 'react'
import { useKeyboardMain, useKeyboardStart } from '../keyboard'
import { BaseHexColor } from '../hex'
import { type displayT, type sizeT, type UseState } from '../types'
import { Palette } from './start/Colors'
import { Modal } from './Modal'
import { Header } from './start/Header'
import { StartButton } from './start/StartButton'
import { SelectMode } from './start/SelectMode'
import { SizeContext } from '../index'

export function StartScreen({
  setSelectedIndex,
  actionOptions,
}: {
  setSelectedIndex: UseState<number>
  actionOptions: (display: displayT) => void
}) {
  const size = useContext<sizeT>(SizeContext)
  const displayLogo = size.height > 13

  useKeyboardStart(actionOptions)
  return (
    <box alignItems="center" justifyContent="center" flexGrow={1} marginTop={1}>
      <Header activate={displayLogo} />
      <StartButton height={size.height} />
      <SelectMode height={size.height} setIndex={setSelectedIndex} />
    </box>
  )
}
