import type { modalState } from '../types'

export function Modal({ activate, state, text }: { activate: boolean; state: modalState; text: string }) {
  return (
    <box
      backgroundColor={state === 'success' ? '#05DF72' : '#DF0505'}
      position="absolute"
      visible={activate}
      top={1}
      right={5}
      width={20}
      height={4}
    >
      <text fg={state === 'success' ? '#0D542B' : '#540D0D'} margin={1}>
        {text}
      </text>
    </box>
  )
}
