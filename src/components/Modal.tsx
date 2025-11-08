export function Modal({ activate }: { activate: boolean }) {
    return (
      <box backgroundColor={'#05df72'} position="absolute" visible={activate} top={1} right={5} width={20} height={2}>
        <text fg={'#0d542b'}>Successfully</text>
      </box>
    )
}
